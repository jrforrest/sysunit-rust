use crate::unit::{Instance};
use crate::error::Error;

mod local;

use self::local::Local;

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

#[derive(Clone, Copy)]
pub enum Operation {
    Check,
    Apply,
    Rollback,
    Deps
}

impl Operation {
    pub fn from_str(operation_name: &str) -> Result<Operation, Error> {
        match operation_name {
            "check" => Ok(Operation::Check),
            "apply" => Ok(Operation::Apply),
            "rollback" => Ok(Operation::Rollback),
            "deps" => Ok(Operation::Deps),
            _ => {
                let err_string = format!("Unkown operation {}", operation_name);
                Err(Error::new(err_string))
            }
        }
    }

    pub fn to_str(&self) -> &'static str{
        match self {
            Operation::Check => "check",
            Operation::Apply => "apply",
            Operation::Rollback => "rollback",
            Operation::Deps => "deps",
        }
    }
}

pub struct Target {
    executor_box: Box<dyn Executor>
}

impl Target {
    pub fn try_new(adapter_name: &str) -> Result<Target, Error> {
        let executor_box: Box<dyn Executor> = match adapter_name {
            "local" => Box::new(Local::new()),
            _ => {
                let error_string = format!("Unknown adapter type: {}", adapter_name);
                return Err(Error::new(error_string));
            }
        };

        Ok(Target { executor_box: executor_box })
    }

    pub fn execute(&self, unit: &Instance, operation_name: &str) -> ExecutionResult {
        let operation = Operation::from_str(operation_name)?;
        self.executor_box.execute(unit, operation)
    }
}
