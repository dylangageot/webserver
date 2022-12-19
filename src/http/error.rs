use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    /// There was an error with i/o
    Io(io::Error),
    /// Request line is malformed
    MalformedRequestLine(String),
    /// Headers are malformed
    MalformedHeaders(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}