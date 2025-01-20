mod bitboard;
mod square;

pub use bitboard::Bitboard;
pub use square::Square;

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
