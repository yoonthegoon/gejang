use crate::error::Error;
use crate::result::Result;
use crate::types::consts::{FILE_A, FILE_H};
use crate::types::enums::Direction;
use crate::types::structs::{Bitboard, Square};
use std::fmt::{Debug, Formatter};
use std::ops::{BitAnd, BitOr, BitOrAssign, BitXor, Not};
use std::str::FromStr;

impl Bitboard {
    pub fn empty() -> Self {
        Bitboard(u64::MIN)
    }

    pub fn universe() -> Self {
        Bitboard(u64::MAX)
    }

    pub fn from_square(square: Square) -> Self {
        Bitboard(1 << square.0)
    }

    #[inline]
    pub fn set_square(&mut self, square_idx: u8) {
        self.0 |= 1u64 << square_idx;
    }

    pub fn ls1b(&self) -> Option<Square> {
        if self.0 == 0 {
            None
        } else {
            Some(Square(self.0.trailing_zeros() as u8))
        }
    }

    pub fn ms1b(&self) -> Option<Square> {
        if self.0 == 0 {
            None
        } else {
            Some(Square(63 - self.0.leading_zeros() as u8))
        }
    }

    pub fn pop_count(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn shift(&self, direction: Direction) -> Bitboard {
        match direction {
            Direction::North => Bitboard(self.0 << 8),
            Direction::South => Bitboard(self.0 >> 8),
            Direction::East => Bitboard((self.0 & !FILE_H.0) << 1),
            Direction::West => Bitboard((self.0 & !FILE_A.0) >> 1),
            Direction::NorthEast => Bitboard((self.0 & !FILE_H.0) << 9),
            Direction::NorthWest => Bitboard((self.0 & !FILE_A.0) << 7),
            Direction::SouthEast => Bitboard((self.0 & !FILE_H.0) >> 7),
            Direction::SouthWest => Bitboard((self.0 & !FILE_A.0) >> 9),
        }
    }
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#018x}", self.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self {
        Bitboard(!self.0)
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
        Ok(Square::new(rank, file))
    }
}
