use crate::board::{Board, Cell};

mod board;
mod defaults;

fn main() {
    let mut board = Board::new(4);
    let _ = board.place_digit(3, Cell { row: 2, col: 3 });
    println!("{}", board.to_string());

    let mut board = Board::new(6);
    let _ = board.place_digit(3, Cell { row: 2, col: 3 });
    println!("{}", board.to_string());

    let mut board = Board::new(9);
    let _ = board.place_digit(3, Cell { row: 2, col: 3 });
    println!("{}", board.to_string());
}
