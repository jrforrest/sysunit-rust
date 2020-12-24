use crate::unit::{Instance};
use crate::error::Error;
use crate::operation::Operation;

mod target;
mod adapter;

pub use self::target::Target;

#[derive(Debug)]
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
    fn init(&mut self) -> Result<(), Error>;
    fn execute(&mut self, unit: &Instance, operation: Operation) -> ExecutionResult;
}
