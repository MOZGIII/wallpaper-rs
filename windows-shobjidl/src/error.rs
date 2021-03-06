use std::fmt;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    NulError(widestring::NulError<u16>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IOError(err) => err.fmt(f),
            Error::NulError(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<widestring::NulError<u16>> for Error {
    fn from(err: widestring::NulError<u16>) -> Error {
        Error::NulError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}
