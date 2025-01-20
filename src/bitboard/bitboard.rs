use std::fmt::{Debug, Formatter};

use super::{Direction, Square};

pub struct Bitboard(u64);

impl Bitboard {
    pub const EMPTY: Self = Bitboard(u64::MIN);
    pub const UNIVERSE: Self = Bitboard(u64::MAX);

    const FILE_A: u64 = 0x0101010101010101;
    const FILE_B: u64 = 0x0202020202020202;
    const FILE_C: u64 = 0x0404040404040404;
    const FILE_D: u64 = 0x0808080808080808;
    const FILE_E: u64 = 0x1010101010101010;
    const FILE_F: u64 = 0x2020202020202020;
    const FILE_G: u64 = 0x4040404040404040;
    const FILE_H: u64 = 0x8080808080808080;

    const RANK_1: u64 = 0x00000000000000FF;
    const RANK_2: u64 = 0x000000000000FF00;
    const RANK_3: u64 = 0x0000000000FF0000;
    const RANK_4: u64 = 0x00000000FF000000;
    const RANK_5: u64 = 0x000000FF00000000;
    const RANK_6: u64 = 0x0000FF0000000000;
    const RANK_7: u64 = 0x00FF000000000000;
    const RANK_8: u64 = 0xFF00000000000000;

    pub fn from_square(square: Square) -> Self {
        Bitboard(1 << square.to_u8())
    }

    pub fn shift(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Bitboard(self.0 << 8),
            Direction::South => Bitboard(self.0 >> 8),
            Direction::East => Bitboard((self.0 & !Self::FILE_H) << 1),
            Direction::West => Bitboard((self.0 & !Self::FILE_A) >> 1),
            Direction::NorthEast => Bitboard((self.0 & !Self::FILE_H) << 9),
            Direction::NorthWest => Bitboard((self.0 & !Self::FILE_A) << 7),
            Direction::SouthEast => Bitboard((self.0 & !Self::FILE_H) >> 7),
            Direction::SouthWest => Bitboard((self.0 & !Self::FILE_A) >> 9),
        }
    }

    pub fn ls1b(&self) -> Option<Square> {
        if self.0 == 0 {
            None
        } else {
            Some(Square::from_u8(self.0.trailing_zeros() as u8).unwrap())
        }
    }

    pub fn ms1b(&self) -> Option<Square> {
        if self.0 == 0 {
            None
        } else {
            Some(Square::from_u8(63 - self.0.leading_zeros() as u8).unwrap())
        }
    }

    pub fn pop_count(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }

    pub fn xor_square(&mut self, square: Square) {
        self.0 ^= 1u64 << square.to_u8();
    }
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#018x}", self.0)
    }
}
