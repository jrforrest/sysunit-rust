use std::collections::HashSet;
use std::net::TcpStream;

use url::Url;
use ssh2::Session;

use crate::error::{BoxedResult, Error};
use crate::unit::Instance;

use super::auth;
use super::execute;
use super::transport;

use crate::operation::Operation;
use crate::execution::Execution;

pub struct Connection {
    session: Session,
    transported_units: HashSet<String>,
}

impl Connection {
    pub fn initialize(url: &Url) -> Result<Connection, Error> {
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

        auth::auth(&mut session, url)?;

        Ok(Connection {
            session: session,
            transported_units:  HashSet::new()
        } )
    }

    pub fn transport(&mut self, unit: &Instance) -> BoxedResult<()> {
        match self.transported_units.get(&unit.id.signature()) {
            Some(_) => Ok(()),
            None => {
                transport::transport(unit, &self.session)?;
                self.transported_units.insert(unit.id.signature());
                Ok(())
            }
        }
    }

    pub fn execute(&mut self, unit: &Instance, operation: Operation) -> BoxedResult<Execution> {
        self.transport(unit)?;

        let remote_path = transport::get_remote_path(unit);

        execute::execute(
            unit,
            &self.session,
            remote_path,
            operation
        )
    }
}
