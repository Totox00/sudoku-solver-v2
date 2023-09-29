use crate::{
    board::{Board, Cell},
    format::format,
};

mod board;
mod defaults;
mod format;

fn main() {
    let mut board = Board::new(4);
    let _ = board.place_digit(3, Cell { row: 2, col: 3 });
    println!("{}", format(&board).unwrap());

    let mut board = Board::new(6);
    let _ = board.place_digit(3, Cell { row: 2, col: 3 });
    println!("{}", format(&board).unwrap());

    let mut board = Board::new(9);
    let _ = board.place_digit(3, Cell { row: 2, col: 3 });
    println!("{}", format(&board).unwrap());
}
