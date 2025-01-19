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
        let mut square_idx = 56; // Start at a8 (7th rank, 0th file)
        let mut parts = s.split('/');

        if s.chars().filter(|&c| c == '/').count() != 7 {
            return Err(Error::ParseError("Invalid number of ranks".to_string()));
        }

        let mut set_piece = |piece_idx: usize, square_idx: &mut u8| {
            ppd[piece_idx].set_square(*square_idx);
            *square_idx += 1;
            1
        };

        for rank in 0..8 {
            let Some(rank_str) = parts.next() else {
                return Err(Error::ParseError("Missing rank data".to_string()));
            };

            let mut file_count = 0;
            let mut chars = rank_str.chars();

            while let Some(c) = chars.next() {
                file_count += match c {
                    'K' => set_piece(0, &mut square_idx),
                    'Q' => set_piece(1, &mut square_idx),
                    'R' => set_piece(2, &mut square_idx),
                    'B' => set_piece(3, &mut square_idx),
                    'N' => set_piece(4, &mut square_idx),
                    'P' => set_piece(5, &mut square_idx),
                    'k' => set_piece(6, &mut square_idx),
                    'q' => set_piece(7, &mut square_idx),
                    'r' => set_piece(8, &mut square_idx),
                    'b' => set_piece(9, &mut square_idx),
                    'n' => set_piece(10, &mut square_idx),
                    'p' => set_piece(11, &mut square_idx),
                    '1'..='8' => {
                        let empty_count = (c as u8) - b'0';
                        square_idx += empty_count;
                        if file_count + empty_count > 8 {
                            return Err(Error::ParseError("Too many squares in rank".to_string()));
                        }
                        empty_count
                    }
                    _ => return Err(Error::ParseError("Invalid character".to_string())),
                };
            }
            if file_count != 8 {
                return Err(Error::ParseError(
                    "Invalid number of squares in rank".to_string(),
                ));
            }
            if rank < 7 {
                square_idx = 48 - (rank * 8);
            }
        }
        if parts.next().is_some() {
            Err(Error::ParseError("Too many ranks".to_string()))
        } else {
            Ok(PiecePlacementData(ppd))
        }
    }
}
