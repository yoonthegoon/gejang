use std::{fmt::Display, str::FromStr};

use crate::{
    bitboard::{Bitboard, Square},
    error::Error,
    result::Result,
};

// K Q R B N P k q r b n p
pub struct PiecePlacementData(pub [Bitboard; 12]);

impl Display for PiecePlacementData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for rank in (0..8).rev() {
            let mut empty_squares = 0;
            for file in 0..8 {
                let square = Square::new(rank, file).unwrap();
                let piece = self.0.iter().enumerate().find_map(|(i, bitboard)| {
                    if bitboard.to_u64() & (1 << square.to_u8()) != 0 {
                        Some(i)
                    } else {
                        None
                    }
                });

                if let Some(piece_index) = piece {
                    if empty_squares > 0 {
                        result.push_str(&empty_squares.to_string());
                        empty_squares = 0;
                    }
                    result.push(match piece_index {
                        0 => 'K',
                        1 => 'Q',
                        2 => 'R',
                        3 => 'B',
                        4 => 'N',
                        5 => 'P',
                        6 => 'k',
                        7 => 'q',
                        8 => 'r',
                        9 => 'b',
                        10 => 'n',
                        11 => 'p',
                        _ => unreachable!(),
                    });
                } else {
                    empty_squares += 1;
                }
            }
            if empty_squares > 0 {
                result.push_str(&empty_squares.to_string());
            }
            if rank > 0 {
                result.push('/');
            }
        }
        write!(f, "{}", result)
    }
}

impl FromStr for PiecePlacementData {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut pieces = [Bitboard::EMPTY; 12];
        let mut rank = 7;
        let mut file = 0;

        for c in s.chars() {
            match c {
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                '1'..='8' => file += c.to_digit(10).unwrap() as u8,
                'K' => pieces[0].xor_square(Square::new(rank, file).unwrap()),
                'Q' => pieces[1].xor_square(Square::new(rank, file).unwrap()),
                'R' => pieces[2].xor_square(Square::new(rank, file).unwrap()),
                'B' => pieces[3].xor_square(Square::new(rank, file).unwrap()),
                'N' => pieces[4].xor_square(Square::new(rank, file).unwrap()),
                'P' => pieces[5].xor_square(Square::new(rank, file).unwrap()),
                'k' => pieces[6].xor_square(Square::new(rank, file).unwrap()),
                'q' => pieces[7].xor_square(Square::new(rank, file).unwrap()),
                'r' => pieces[8].xor_square(Square::new(rank, file).unwrap()),
                'b' => pieces[9].xor_square(Square::new(rank, file).unwrap()),
                'n' => pieces[10].xor_square(Square::new(rank, file).unwrap()),
                'p' => pieces[11].xor_square(Square::new(rank, file).unwrap()),
                _ => return Err(Error::ParseError(s.to_string())),
            }
            if c != '/' && !c.is_ascii_digit() {
                file += 1;
            }
        }

        Ok(PiecePlacementData(pieces))
    }
}
