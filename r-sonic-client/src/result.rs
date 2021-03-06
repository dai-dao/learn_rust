use std::error::Error as StdError;
use std::fmt;


pub type Result<T> = std::result::Result<T, Error>;


// implement StdError trait
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl StdError for Error {}

impl Error  {
    pub fn new(kind: ErrorKind) -> Self {
        Error { kind }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    ConnectToServer,
    WriteStream,
    ReadStream,
    WrongSonicResponse,
    SwitchMode,
    QueryResponseError(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        match self.kind {
            ErrorKind::ConnectToServer => write!(f, "{}", "Can not connect to Sonic server"),
            ErrorKind::WriteStream => write!(f, "{}", "Can not write to stream"),
            ErrorKind::ReadStream => write!(f, "{}", "Can not read from stream"),
            ErrorKind::WrongSonicResponse => write!(f, "{}", "Wrong Sonic response for command"),
            ErrorKind::SwitchMode => write!(f, "{}", "wrong mode"),
            ErrorKind::QueryResponseError(message) => {
                write!(f, "Error in query response: {}", message)
            }
        }
    }
}
