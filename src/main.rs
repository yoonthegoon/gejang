use crate::position::Position;

mod bitboard;
mod error;
mod move_generation;
mod position;
mod result;

fn main() {
    let game = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    println!("{:?}", game);
}
