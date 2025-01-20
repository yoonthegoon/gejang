use std::{fmt::Display, str::FromStr};

use crate::{bitboard::Square, error::Error, result::Result};

use super::{
    active_color::ActiveColor, castling_availability::CastlingAvailability,
    piece_placement_data::PiecePlacementData,
};

pub struct Position {
    pub piece_placement_data: PiecePlacementData,
    pub active_color: ActiveColor,
    pub castling_availability: CastlingAvailability,
    pub en_passant_target_square: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}

impl Position {
    pub fn new() -> Self {
        Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn from_fen(fen: &str) -> Result<Self> {
        fen.parse()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {}",
            self.piece_placement_data,
            self.active_color,
            self.castling_availability,
            self.en_passant_target_square
                .map_or("-".to_string(), |square| square.to_string()),
            self.halfmove_clock,
            self.fullmove_number,
        )
    }
}

impl FromStr for Position {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();
        let piece_placement_data = parts
            .next()
            .ok_or(Error::ParseError(s.to_string()))?
            .parse()?;
        let active_color = parts
            .next()
            .ok_or(Error::ParseError(s.to_string()))?
            .parse()?;
        let castling_availability = parts
            .next()
            .ok_or(Error::ParseError(s.to_string()))?
            .parse()?;
        let en_passant_target_square = match parts.next().ok_or(Error::ParseError(s.to_string()))? {
            "-" => None,
            s => Some(s.parse()?),
        };
        let halfmove_clock = parts
            .next()
            .ok_or(Error::ParseError(s.to_string()))?
            .parse()
            .map_err(|_| Error::ParseError(s.to_string()))?;
        let fullmove_number = parts
            .next()
            .ok_or(Error::ParseError(s.to_string()))?
            .parse()
            .map_err(|_| Error::ParseError(s.to_string()))?;

        Ok(Position {
            piece_placement_data,
            active_color,
            castling_availability,
            en_passant_target_square,
            halfmove_clock,
            fullmove_number,
        })
    }
}
