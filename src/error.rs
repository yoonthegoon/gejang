use std::num::ParseIntError;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    ParseError(String),
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseError(value.to_string())
    }
}
