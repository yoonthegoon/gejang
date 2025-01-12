use crate::error::Error;
use crate::result::Result;
use crate::types::enums::ActiveColor;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl Display for ActiveColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ActiveColor::White => "w".to_string(),
            ActiveColor::Black => "b".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for ActiveColor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "w" => Ok(ActiveColor::White),
            "b" => Ok(ActiveColor::Black),
            _ => Err(Error::ParseError(s.to_string())),
        }
    }
}
