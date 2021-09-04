use std::{error, fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

use crate::parser::Rule;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Parser(pest::error::Error<Rule>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "{:?}", self)
        }

    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<io::Error> for Error {
    fn from(item: io::Error) -> Self {
        Error::IO(item)
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(item: pest::error::Error<Rule>) -> Self {
        Error::Parser(item)
    }
}
