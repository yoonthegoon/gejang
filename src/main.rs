use crate::position::Position;

mod bitboard;
mod error;
mod position;
mod result;

fn main() {
    let game = Position::new();
    println!("{}", game);
}
