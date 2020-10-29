use crate::error::Error;

#[derive(Clone, Copy, Debug)]
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
