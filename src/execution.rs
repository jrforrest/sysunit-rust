use crate::unit::{Instance};
use crate::error::Error;
use crate::operation::Operation;

mod local;
mod adapter;

use self::local::Local;
use self::adapter::Adapter;

pub struct Execution {
    pub unit_name: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

impl Execution {
    pub fn success(&self) -> bool {
        self.exit_code == 0
    }
}

pub type ExecutionResult = Result<Execution, Error>;

pub trait Executor {
    fn execute(&self, unit: &Instance, operation: Operation) -> ExecutionResult;
}

pub struct Target {
    executor_box: Box<dyn Executor>
}

impl Target {
    pub fn try_new(adapter_name: &str) -> Result<Target, Error> {
        let executor_box: Box<dyn Executor> = match adapter_name {
            "local" => Box::new(Local::new()),
            adapter_name => Box::new(Adapter::try_new(adapter_name)?),
        };

        Ok(Target { executor_box: executor_box })
    }

    pub fn execute(&self, unit: &Instance, operation: Operation) -> ExecutionResult {
        self.executor_box.execute(unit, operation)
    }
}
