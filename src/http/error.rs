use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    /// There was an error with i/o
    Io(io::Error),
    /// Request line is malformed
    MalformedRequestLine(String),
    /// Headers are malformed
    MalformedHeaders(String),
    /// Rendering of index page failed
    IndexRendering(ramhorns::Error),
    IndexGeneration(String)
    
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{}", e),
            Error::MalformedRequestLine(r) => write!(f, "malformed request line {}", r),
            Error::MalformedHeaders(r) => write!(f, "malformed headers {}", r),
            Error::IndexRendering(e) => write!(f, "failed rendering index {}", e),
            Error::IndexGeneration(s) => write!(f, "failed generating index {}", s),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<ramhorns::Error> for Error {
    fn from(e: ramhorns::Error) -> Self {
        Error::IndexRendering(e)
    }
}
