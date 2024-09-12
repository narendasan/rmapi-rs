use reqwest;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ReqwestError(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref err) => write!(f, "Io error: {}", err),
            Error::ReqwestError(ref err) => {
                let reqwest_error: &reqwest::Error = err;
                return write!(f, "Reqwest error: {}, caused by: {}", reqwest_error, reqwest_error.source().unwrap())
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::IoError(ref err) => Some(err),
            Error::ReqwestError(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ReqwestError(err)
    }
}
