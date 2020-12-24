#![macro_use]

use std::fmt;

pub type BoxedResult<T> = Result<T, Box<dyn std::error::Error>>;

#[macro_export]

macro_rules! wrap_error {
    ($format_string: literal, $error: expr) => {
        Error::new(format!($format_string, $error.to_string()));
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub msg: String
}

impl Error {
    pub fn new(msg: String) -> Error {
        Error{msg: msg}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sysunit Error: {}", self.msg)
    }
}

impl std::error::Error for Error {}
