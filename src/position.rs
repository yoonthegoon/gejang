use crate::bitboard::{Bitboard, Square};
use crate::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct CastlingAvailability {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}

/// K Q R B N P k q r b n p
#[derive(Debug)]
pub struct PiecePlacementData(pub(crate) [Bitboard; 12]);

#[derive(Debug)]
pub struct Position {
    pub piece_placement_data: PiecePlacementData,
    pub active_color: ActiveColor,
    pub castling_availability: CastlingAvailability,
    pub en_passant_target_square: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}

#[derive(Debug)]
pub enum ActiveColor {
    White,
    Black,
}

impl Position {
    pub(crate) fn new() -> Self {
        Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn from_fen(fen: &str) -> Result<Self, Error> {
        fen.parse()
    }
}

impl Display for ActiveColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ActiveColor::White => "w".to_string(),
            ActiveColor::Black => "b".to_string(),
        };
        write!(f, "{}", str)
    }
}

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

impl Display for PiecePlacementData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {}",
            self.piece_placement_data,
            self.active_color,
            self.castling_availability,
            self.en_passant_target_square
                .map_or_else(|| "-".to_string(), |s| s.to_string()),
            self.halfmove_clock,
            self.fullmove_number
        )
    }
}

impl FromStr for ActiveColor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(ActiveColor::White),
            "b" => Ok(ActiveColor::Black),
            _ => Err(Error::ParseError(s.to_string())),
        }
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

impl FromStr for PiecePlacementData {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

impl FromStr for Position {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let piece_placement_data = parts.next().ok_or(Error::ParseError(s.to_string()))?;
        let active_color = parts.next().ok_or(Error::ParseError(s.to_string()))?;
        let castling_availability = parts.next().ok_or(Error::ParseError(s.to_string()))?;
        let en_passant_target_square = parts.next().ok_or(Error::ParseError(s.to_string()))?;
        let halfmove_clock = parts.next().ok_or(Error::ParseError(s.to_string()))?;
        let fullmove_number = parts.next().ok_or(Error::ParseError(s.to_string()))?;

        Ok(Position {
            piece_placement_data: piece_placement_data.parse()?,
            active_color: active_color.parse()?,
            castling_availability: castling_availability.parse()?,
            en_passant_target_square: if en_passant_target_square == "-" {
                None
            } else {
                Some(en_passant_target_square.parse()?)
            },
            halfmove_clock: halfmove_clock.parse()?,
            fullmove_number: fullmove_number.parse()?,
        })
    }
}
