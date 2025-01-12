use crate::types::structs::Position;

mod error;
mod impls;
mod result;
mod types;

fn main() {
    let game = Position::new();
    println!("{:?}", game);
}
