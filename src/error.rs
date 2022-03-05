use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ParseRowError(String),
    ParseTableError(String),
    MonthParseError(String),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match &self {
            Error::ParseRowError(s) => s,
            Error::ParseTableError(s) => s,
            Error::MonthParseError(s) => s,
        };

        write!(f, "{}", msg)
    }
}
