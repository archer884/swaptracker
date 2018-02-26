use std::error;
use std::fmt;
use std::io;
use std::result;
use sys;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    description: &'static str,
}

#[derive(Debug)]
pub enum ErrorKind {
    Io(io::Error),
    Sys(sys::Error),
}

impl Error {
    pub fn io(e: io::Error, message: &'static str) -> Self {
        Self {
            kind: ErrorKind::Io(e),
            description: message,
        }
    }

    pub fn sys(e: sys::Error) -> Self {
        Self {
            kind: ErrorKind::Sys(e),
            description: "Failed to get system info",
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.description
    }

    fn cause(&self) -> Option<&error::Error> {
        use self::ErrorKind::*;

        match self.kind {
            Io(ref error) => Some(error),
            Sys(ref error) => Some(error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;

        write!(f, "{}", self.description)?;
        if let Some(cause) = self.cause() {
            write!(f, "  {}", cause)?;
        }

        Ok(())
    }
}
