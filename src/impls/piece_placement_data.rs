use crate::error::Error;
use crate::result::Result;
use crate::types::structs::{Bitboard, PiecePlacementData, Square};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl Display for PiecePlacementData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for rank in (0..8).rev() {
            let mut empty_count = 0;

            for file in 0..8 {
                let square = Square::new(rank, file);
                let mut piece_found = false;

                for (i, bitboard) in self.0.iter().enumerate() {
                    if bitboard.0 & (1 << square.0) != 0 {
                        if empty_count > 0 {
                            result.push_str(&empty_count.to_string());
                            empty_count = 0;
                        }
                        let piece = match i {
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
                        };
                        result.push(piece);
                        piece_found = true;
                        break;
                    }
                }
                if !piece_found {
                    empty_count += 1;
                }
            }
            if empty_count > 0 {
                result.push_str(&empty_count.to_string());
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
        let mut ppd = [Bitboard::empty(); 12];
        let mut rank = 7;
        for line in s.lines() {
            let mut file = 0;
            for c in line.chars() {
                match c {
                    'K' => ppd[0] |= Bitboard::from_square(Square::new(rank, file)),
                    'Q' => ppd[1] |= Bitboard::from_square(Square::new(rank, file)),
                    'R' => ppd[2] |= Bitboard::from_square(Square::new(rank, file)),
                    'B' => ppd[3] |= Bitboard::from_square(Square::new(rank, file)),
                    'N' => ppd[4] |= Bitboard::from_square(Square::new(rank, file)),
                    'P' => ppd[5] |= Bitboard::from_square(Square::new(rank, file)),
                    'k' => ppd[6] |= Bitboard::from_square(Square::new(rank, file)),
                    'q' => ppd[7] |= Bitboard::from_square(Square::new(rank, file)),
                    'r' => ppd[8] |= Bitboard::from_square(Square::new(rank, file)),
                    'b' => ppd[9] |= Bitboard::from_square(Square::new(rank, file)),
                    'n' => ppd[10] |= Bitboard::from_square(Square::new(rank, file)),
                    'p' => ppd[11] |= Bitboard::from_square(Square::new(rank, file)),
                    '1'..='8' => {
                        file += c.to_digit(10).unwrap() as u8;
                        continue;
                    }
                    '/' => {
                        if file != 8 {
                            return Err(Error::ParseError(s.to_string()));
                        }
                        rank -= 1;
                        file = 0;
                        continue;
                    }
                    _ => return Err(Error::ParseError(s.to_string())),
                }
                file += 1;
            }
        }
        // TODO: check all squares have been gone over
        Ok(PiecePlacementData(ppd))
    }
}
