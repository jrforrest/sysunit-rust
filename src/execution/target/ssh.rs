use super::super::{Executor, Operation, Execution};
use crate::unit::Instance;
use crate::error::{Error};

use url::Url;

mod auth;
mod transport;
mod execute;
mod connection;
mod close_channel;

use connection::Connection;

pub struct SSH {
    connection: Option<Connection>,
    url: Option<Url>
}

impl SSH { 
    pub fn new(url: Option<Url>) -> SSH {
        SSH { connection: None, url: url }
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
            None => {
                let initialized_connection = Connection::initialize(&url)?;
                self.connection = Some(initialized_connection)
            },
            Some(_) => ()
        };

        Ok(())
    }

    fn execute(&mut self, unit: &Instance, operation: Operation) -> Result<Execution, Error> {
        match &mut self.connection {
            None => return Err(Error::new(format!(
                "Attempted to execute on unintialized SSH session."
            ))),
            Some(connection) => {
                let execution = connection.execute(unit, operation)
                    .map_err(|e| wrap_error!("SSH Execution Error: {}", e));

                execution
            }
        }
    }
}
