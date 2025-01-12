#[derive(Debug)]
pub enum ActiveColor {
    White,
    Black,
}

pub enum Direction {
    North = 8,
    South = -8,
    East = -1,
    West = 1,
    NorthEast = 9,
    NorthWest = 7,
    SouthEast = -7,
    SouthWest = -9,
}
