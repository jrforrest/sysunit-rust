#[derive(Debug)]
pub struct Error {
    pub msg: String
}

impl Error {
    pub fn new(msg: String) -> Error {
        Error{msg: msg}
    }
}
