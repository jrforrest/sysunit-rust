use url::{Url};

use crate::error::Error;
use crate::operation::Operation;
use crate::unit::Instance;
use super::ExecutionResult;

mod local;
mod ssh;

use self::ssh::SSH;
use self::local::Local;
use super::adapter::Adapter;
use super::Executor;

pub struct Target {
    executor: Box<dyn Executor>
}

impl Target {
    pub fn try_new(url_str: Option<&str>, adapter_name: Option<&str>) -> Result<Target, Error> {
        let parsed_url_option = match url_str {
            None => None,
            Some(u) => Some(
                Url::parse(u).map_err(|e| Error::new(format!("{}", e)))?
            ),
        };

        let adapter_name = match adapter_name {
            Some(n) => n,
            None => match parsed_url_option {
                None => "local",
                Some(ref parsed_url) => parsed_url.scheme()
            }
        };

        let mut executor: Box<dyn Executor> = match adapter_name {
            "local" => Box::new(Local::new(parsed_url_option.clone())),
            "ssh" => Box::new(SSH::new(parsed_url_option.clone())),
            adapter_name => Box::new(Adapter::try_new(adapter_name)?),
        };

        executor.init()?;

        Ok(Target { executor: executor })
    }

    pub fn execute(&mut self, unit: &Instance, operation: Operation) -> ExecutionResult {
        self.executor.execute(unit, operation)
    }
}
