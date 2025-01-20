use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::{error::Error, result::Result};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Square(u8);

impl Square {
    pub fn new(rank: u8, file: u8) -> Option<Self> {
        if rank < 8 && file < 8 {
            Some(Square(rank * 8 + file))
        } else {
            None
        }
    }

    #[allow(dead_code)] // TODO: remove
    pub fn from_u8(i: u8) -> Option<Self> {
        if i < 64 {
            Some(Square(i))
        } else {
            None
        }
    }

    pub fn file(&self) -> u8 {
        self.0 % 8
    }

    pub fn rank(&self) -> u8 {
        self.0 / 8
    }

    pub fn to_u8(&self) -> u8 {
        self.0
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let file = match self.file() {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => unreachable!(),
        };
        let rank = self.rank() + 1;
        write!(f, "{}{}", file, rank)
    }
}

impl FromStr for Square {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 2 {
            return Err(Error::ParseError(s.to_string()));
        }
        let file = match s.chars().nth(0).unwrap() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return Err(Error::ParseError(s.to_string())),
        };
        let rank = match s.chars().nth(1).unwrap() {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return Err(Error::ParseError(s.to_string())),
        };
        Ok(Square(file + rank * 8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        for rank in 0..8 {
            for file in 0..8 {
                let square = Square::new(rank, file);
                assert!(square.is_some());
                assert_eq!(square.unwrap().rank(), rank);
                assert_eq!(square.unwrap().file(), file);
            }
        }
        assert!(Square::new(8, 0).is_none());
        assert!(Square::new(0, 8).is_none());
        assert!(Square::new(8, 8).is_none());
    }

    #[test]
    fn test_from_u8() {
        for i in 0..64 {
            let square = Square::from_u8(i);
            assert!(square.is_some());
            assert_eq!(square.unwrap().to_u8(), i);
        }
        assert!(Square::from_u8(64).is_none());
        assert!(Square::from_u8(255).is_none());
    }

    #[test]
    fn test_file() {
        for i in 0..64 {
            let square = Square(i);
            assert_eq!(square.file(), i % 8);
        }
    }

    #[test]
    fn test_rank() {
        for i in 0..64 {
            let square = Square(i);
            assert_eq!(square.rank(), i / 8);
        }
    }

    #[test]
    fn test_to_u8() {
        for i in 0..64 {
            let square = Square(i);
            assert_eq!(square.to_u8(), i);
        }
    }

    #[test]
    fn test_display() {
        for rank in 0..8 {
            for file in 0..8 {
                let square = Square(file + rank * 8);
                let expected = format!("{}{}", ('a' as u8 + file) as char, rank + 1);
                assert_eq!(format!("{}", square), expected);
            }
        }
    }

    #[test]
    fn test_parse() {
        for rank in 0..8 {
            for file in 0..8 {
                let square = Square(file + rank * 8);
                let expected = format!("{}{}", ('a' as u8 + file) as char, rank + 1);
                assert_eq!(Square::from_str(&expected).unwrap(), square);
            }
        }
        assert!(Square::from_str("").is_err());
        assert!(Square::from_str("a").is_err());
        assert!(Square::from_str("a1b").is_err());
        assert!(Square::from_str("i1").is_err());
        assert!(Square::from_str("01").is_err());
        assert!(Square::from_str("a0").is_err());
        assert!(Square::from_str("a9").is_err());
    }
}
