mod bitboard;
mod square;

pub use bitboard::Bitboard;
pub use square::Square;

#[allow(dead_code)] // TODO: remove
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}
