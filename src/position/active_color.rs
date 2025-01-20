use std::{fmt::Display, str::FromStr};

use crate::error::Error;

pub enum ActiveColor {
    White,
    Black,
}

impl Display for ActiveColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActiveColor::White => write!(f, "w"),
            ActiveColor::Black => write!(f, "b"),
        }
    }
}

impl FromStr for ActiveColor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(ActiveColor::White),
            "b" => Ok(ActiveColor::Black),
            _ => Err(Error::ParseError(format!("Invalid ActiveColor: {}", s))),
        }
    }
}
