use crate::types::enums::ActiveColor;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Bitboard(pub u64);

#[derive(Debug)]
pub struct CastlingAvailability {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}

/// K Q R B N P k q r b n p
#[derive(Debug)]
pub struct PiecePlacementData(pub [Bitboard; 12]);

#[derive(Debug)]
pub struct Position {
    pub piece_placement_data: PiecePlacementData,
    pub active_color: ActiveColor,
    pub castling_availability: CastlingAvailability,
    pub en_passant_target_square: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Square(pub u8);
