use super::super::{Executor, Operation, Execution};

use crate::unit::Instance;
use crate::error::{Error, BoxedResult};
use crate::ui::prompt_ssh_password;

use std::path::Path;
use std::fs::{File};
use std::net::TcpStream;
use std::io::prelude::*;

use url::Url;
use log::{debug, info};
use ssh2::Session;

macro_rules! wrap_error {
    ($format_string: literal, $error: expr) => {
        Error::new(format!($format_string, $error.to_string()));
    }
}

pub struct SSH {
    connection: Connection,
    url: Option<Url>
}

impl SSH { 
    pub fn new(url: Option<Url>) -> SSH {
        SSH { connection: Connection::Uninitialized, url: url }
    }

    fn transport(&self, unit: &Instance, session: &Session) -> BoxedResult<()> {
        let dest_path = Path::new("/tmp/unit");

        let definition = &*unit.definition_rc;
        let mut local_file = File::open(Path::new(&definition.path)).map_err(|e| Box::new(e))?;

        let mut local_buf = Vec::new();
        local_file.read_to_end(&mut local_buf).map_err(|e| Box::new(e))?;

        use std::convert::TryFrom;
        let mut remote_file = session.scp_send(dest_path,
            0o755,
            u64::try_from(local_buf.len()).unwrap(),
            None
        ).map_err(|e| Box::new(e))?;

        remote_file.write(&local_buf).map_err(|e| Box::new(e))?;
        remote_file.flush().map_err(|e| Box::new(e))?;

        Ok(())
    }
}

impl Executor for SSH {
    fn init(&mut self) -> Result<(), Error> {
        let url = match self.url {
            None => return Err(Error::new(format!(
                "Target URL must be provided for SSH"
            ))),
            Some(ref x) => x
        };

        match self.connection {
            Connection::Uninitialized => {
                let started_connection = StartedConnection::initialize(&url)?;
                self.connection = Connection::Initialized(started_connection)
            },
            Connection::Initialized(_) => ()
        };

        Ok(())
    }

    fn execute(&self, unit: &Instance, operation: Operation) -> Result<Execution, Error> {
        let session = match self.connection {
            Connection::Uninitialized => return Err(Error::new(format!(
                "Attempted to execute on unintialized SSH session."
            ))),
            Connection::Initialized(ref connection) => connection.get_session()
        };

        self.transport(unit, session).map_err(|e| {
            wrap_error!("Transport Error: {}", e)
        })?;

        let mut channel = session.channel_session().map_err(|e| {
            wrap_error!("Channel Initialization Error: {}", e)
        })?;

        let env_vars = unit.id.args.vec.iter().map(|arg|
            format!("{}=\"{}\"", arg.name, arg.value)
        );

        let joined_env_vars = env_vars.collect::<Vec<String>>().join("  ");

        let command_string = format!("/tmp/unit {} {}", operation.to_str(), joined_env_vars);

        debug!("command string: {}", command_string);

        channel.exec(command_string.as_str()).map_err(|e|
            wrap_error!("Failed to execute unit: {}", e)
        )?;

        channel.send_eof().map_err(|e|
            wrap_error!("Failed to send EOF on channel: {}", e)
        )?;

        channel.close().map_err(|e|
            wrap_error!("Failed to send close on channel: {}", e)
        )?;

        let exit_code = channel.exit_status()
            .map_err(|e| Error::new(format!(
                "Failed to get exit code: {}", e
            )))?;

        let mut output = String::new();
        let mut stderr = String::new();

        loop {
            channel.wait_eof().map_err(|e| {
                debug!("Wait EOF on Channel error: {:?}", e);
                Error::new(format!("Failed to wait for EOF on channel: {}", e))
            })?;

            channel.read_to_string(&mut output).map_err(|e| {
                debug!("Command output read error: {:?}", e);
                Error::new(format!("Failed to read unit output: {}", e))
            })?;

            channel.stderr().read_to_string(&mut stderr).map_err(|e| {
                debug!("Command stderr read error: {:?}", e);
                Error::new(format!("Failed to read unit stderr: {}", e))
            })?;

            if channel.eof() { break };
        }

        debug!("eof: {:?}", channel.wait_eof());

        let execution = Execution {
            unit_name: unit.definition_rc.name.clone(),
            exit_code: exit_code,
            stdout: output,
            stderr: stderr
        };

        debug!("execution: {:?}", execution);

        Ok(execution)
    }
}

pub enum Connection {
    Initialized(StartedConnection),
    Uninitialized
}

pub struct StartedConnection {
    session: Session,
}

impl StartedConnection {
    pub fn initialize(url: &Url) -> Result<StartedConnection, Error> {
        let addrs = url.socket_addrs(||
            match url.scheme() {
                "ssh" => Some(22),
                _ => None
            }
        ).map_err(|e| Error::new(format!("Address Resolution Error on `{}`: {}", url, e)))?;

        let tcp = TcpStream::connect(&*addrs)
            .map_err(|e| Error::new(format!("TCP Error: {}", e)))?;

        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);
        session.handshake().map_err(|e| Error::new(format!("SSH Handshake Error: {}", e)))?;

        auth(&mut session, url)?;

        Ok(StartedConnection {
            session: session
        } )
    }

    pub fn get_session(&self) -> &Session {
        &self.session
    }
}

fn auth(session: &mut Session, url: &Url) -> Result<(), Error> {
    let username = match url.username() {
        "" => return Err(
            Error::new(format!("Target `{:?}` must specify a username", url))
        ),
        v => v
    };

    fn auth_via_agent(session: &mut Session, username: &str) -> BoxedResult<()> {
        let mut agent = session.agent().map_err(|e| Error::new(e.to_string()))?;
        agent.connect()?;
        agent.list_identities()?;
        let identities = agent.identities().map_err(|e| Box::new(e))?;

        for identity in identities {
            match agent.userauth(username, &identity) {
                Err(_) => (),
                Ok(_) => return Ok(())
            }
        }

        return Err(Box::new(Error::new(format!("No identiy in agent could authenticate"))))
    }

    fn auth_via_password(
        session: &mut Session,
        url: &Url,
        username: &str
    ) -> BoxedResult<()> {
        let host = match url.host() {
            None => return Err(Box::new(Error::new(format!("URL {} must specify a host!", url)))),
            Some(x) => x.to_string()
        };

        let password = prompt_ssh_password(username, &host)?;
        match session.userauth_password(username, &password) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e))
        }
    }

    match auth_via_agent(session, username) {
        Ok(_) => return Ok(()),
        Err(e) => info!("SSH agent auth to {:?} failed: {}", url, e)
    }

    match auth_via_password(session, url, username) {
        Ok(_) => return Ok(()),
        Err(e) => info!("SSH password auth to {:?} failed: {}", url, e)
    }

    Err(Error::new(format!("SSH Agent and Password auth to {:?} failed", url)))
}
