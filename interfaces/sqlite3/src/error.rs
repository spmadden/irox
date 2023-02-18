use std::fmt::Display;


#[derive(Debug, Clone)]
pub struct Error {
    msg: String
}

impl Error {
    pub fn new(msg: &str) -> Error{
        Error {msg: msg.to_string()}
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Error {

}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error { msg: format!("IOE: {}", value.to_string()) }
    }
}