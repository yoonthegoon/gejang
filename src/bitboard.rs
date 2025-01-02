use std::fmt::{Debug, Formatter};
use std::ops::{BitAnd, BitOr, BitXor, Not};

const FILE_A: Bitboard = Bitboard(0x0101010101010101);
const FILE_B: Bitboard = Bitboard(0x0202020202020202);
const FILE_C: Bitboard = Bitboard(0x0404040404040404);
const FILE_D: Bitboard = Bitboard(0x0808080808080808);
const FILE_E: Bitboard = Bitboard(0x1010101010101010);
const FILE_F: Bitboard = Bitboard(0x2020202020202020);
const FILE_G: Bitboard = Bitboard(0x4040404040404040);
const FILE_H: Bitboard = Bitboard(0x8080808080808080);

const RANK_1: Bitboard = Bitboard(0x00000000000000FF);
const RANK_2: Bitboard = Bitboard(0x000000000000FF00);
const RANK_3: Bitboard = Bitboard(0x0000000000FF0000);
const RANK_4: Bitboard = Bitboard(0x00000000FF000000);
const RANK_5: Bitboard = Bitboard(0x000000FF00000000);
const RANK_6: Bitboard = Bitboard(0x0000FF0000000000);
const RANK_7: Bitboard = Bitboard(0x00FF000000000000);
const RANK_8: Bitboard = Bitboard(0xFF00000000000000);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Bitboard(pub u64);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Square(pub u8);

pub enum Direction {
    North = 8,
    South = -8,
    East = -1,
    West = 1,
    NorthEast = 9,
    NorthWest = 7,
    SouthEast = -7,
    SouthWest = -9,
}

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

    pub fn ls1b(&self) -> Option<Square> {
        if self.0 == 0 {
            return None;
        }
        Some(Square(self.0.trailing_zeros() as u8))
    }

    pub fn ms1b(&self) -> Option<Square> {
        if self.0 == 0 {
            return None;
        }
        Some(Square(63 - self.0.leading_zeros() as u8))
    }

    pub fn pop_count(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn shift(&self, direction: Direction) -> Bitboard {
        match direction {
            Direction::North => Bitboard((self.0 & 0x00FFFFFFFFFFFFFF) << 8),
            Direction::South => Bitboard((self.0 & 0xFFFFFFFFFFFFFF00) >> 8),
            Direction::East => Bitboard((self.0 & 0x7F7F7F7F7F7F7F7F) << 1),
            Direction::West => Bitboard((self.0 & 0xFEFEFEFEFEFEFEFE) >> 1),
            Direction::NorthEast => Bitboard((self.0 & 0x007F7F7F7F7F7F7F) << 9),
            Direction::NorthWest => Bitboard((self.0 & 0x00FEFEFEFEFEFEFE) << 7),
            Direction::SouthEast => Bitboard((self.0 & 0x7F7F7F7F7F7F7F00) >> 7),
            Direction::SouthWest => Bitboard((self.0 & 0xFEFEFEFEFEFEFE00) >> 9),
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
