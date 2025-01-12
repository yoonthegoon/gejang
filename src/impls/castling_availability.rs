use crate::error::Error;
use crate::result::Result;
use crate::types::structs::CastlingAvailability;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl Display for CastlingAvailability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        if self.white_king_side {
            str.push('K');
        }
        if self.white_queen_side {
            str.push('Q');
        }
        if self.black_king_side {
            str.push('k');
        }
        if self.black_queen_side {
            str.push('q');
        }
        if str.is_empty() {
            str.push('-');
        }
        write!(f, "{}", str)
    }
}

impl FromStr for CastlingAvailability {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
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
