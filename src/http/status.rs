
#[derive(Debug, PartialEq)]
pub enum Status {
    Ok = 200,
    Accepted = 202,
    NotFound = 404,
}

use std::fmt;
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Ok => String::from("200 OK"),
            Self::Accepted => String::from("202 Accepted"),
            Self::NotFound => String::from("404 Not Found"),
        })
    }
}