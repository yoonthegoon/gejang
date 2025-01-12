use crate::error::Error;
use crate::result::Result;
use crate::types::structs::Position;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl Position {
    pub(crate) fn new() -> Self {
        Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn from_fen(fen: &str) -> Result<Self> {
        fen.parse()
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

impl FromStr for Position {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
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
