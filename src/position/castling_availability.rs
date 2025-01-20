use std::{fmt::Display, str::FromStr};

use crate::error::Error;

pub struct CastlingAvailability {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}

impl Display for CastlingAvailability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        if self.white_king_side {
            buf.push('K');
        }
        if self.white_queen_side {
            buf.push('Q');
        }
        if self.black_king_side {
            buf.push('k');
        }
        if self.black_queen_side {
            buf.push('q');
        }
        if buf.is_empty() {
            buf.push('-');
        }
        write!(f, "{}", buf)
    }
}

impl FromStr for CastlingAvailability {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut white_king_side = false;
        let mut white_queen_side = false;
        let mut black_king_side = false;
        let mut black_queen_side = false;
        for c in s.chars() {
            match c {
                '-' => break,
                'K' => white_king_side = true,
                'Q' => white_queen_side = true,
                'k' => black_king_side = true,
                'q' => black_queen_side = true,
                _ => return Err(Error::ParseError(s.to_string())),
            }
        }
        Ok(CastlingAvailability {
            white_king_side,
            white_queen_side,
            black_king_side,
            black_queen_side,
        })
    }
}
