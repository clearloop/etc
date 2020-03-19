//! etc errors

use std::{
    convert::From,
    error::Error as StdError,
    fmt,
    fmt::{Debug, Display},
    io::Error as IoError,
};

/// etc Error
#[derive(Debug)]
pub enum Error {
    /// custom error type in etc
    Custom(String),
    /// io error transport
    IoError(String),
}

/// support errors
impl From<IoError> for Error {
    fn from(e: IoError) -> Error {
        Error::Custom(e.to_string())
    }
}

/// support {} without verbose wrapper
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Custom(s) => write!(f, "{}", s),
            Error::IoError(s) => write!(f, "{}", s),
        }
    }
}

/// implement custom std error trait for Error
impl StdError for Error {}
