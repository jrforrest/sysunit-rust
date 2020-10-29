use std::fmt;

pub type BoxedResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type ErrorResult<T> = Result<T, Error>;

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
