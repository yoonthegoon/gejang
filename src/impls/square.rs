use crate::types::structs::Square;
use std::fmt::{Display, Formatter};

impl Square {
    pub fn new(rank: u8, file: u8) -> Self {
        Square(rank * 8 + file)
    }

    pub fn rank(&self) -> u8 {
        self.0 / 8
    }

    pub fn file(&self) -> u8 {
        self.0 % 8
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
