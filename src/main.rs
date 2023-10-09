use crate::{
    board::{Board, Cell},
    format::format,
};

mod board;
mod defaults;
mod format;

fn main() {
    let mut board = Board::new(9);
    board.place_digit(1, Cell { row: 0, col: 3 });
    board.place_digit(5, Cell { row: 0, col: 5 });
    board.place_digit(1, Cell { row: 1, col: 0 });
    board.place_digit(4, Cell { row: 1, col: 1 });
    board.place_digit(6, Cell { row: 1, col: 6 });
    board.place_digit(7, Cell { row: 1, col: 7 });
    board.place_digit(8, Cell { row: 2, col: 1 });
    board.place_digit(2, Cell { row: 2, col: 5 });
    board.place_digit(4, Cell { row: 2, col: 6 });
    board.place_digit(6, Cell { row: 3, col: 1 });
    board.place_digit(3, Cell { row: 3, col: 2 });
    board.place_digit(7, Cell { row: 3, col: 4 });
    board.place_digit(1, Cell { row: 3, col: 7 });
    board.place_digit(9, Cell { row: 4, col: 0 });
    board.place_digit(3, Cell { row: 4, col: 8 });
    board.place_digit(1, Cell { row: 5, col: 1 });
    board.place_digit(9, Cell { row: 5, col: 4 });
    board.place_digit(5, Cell { row: 5, col: 6 });
    board.place_digit(2, Cell { row: 5, col: 7 });
    board.place_digit(7, Cell { row: 6, col: 2 });
    board.place_digit(2, Cell { row: 6, col: 3 });
    board.place_digit(8, Cell { row: 6, col: 7 });
    board.place_digit(2, Cell { row: 7, col: 1 });
    board.place_digit(6, Cell { row: 7, col: 2 });
    board.place_digit(3, Cell { row: 7, col: 7 });
    board.place_digit(5, Cell { row: 7, col: 8 });
    board.place_digit(4, Cell { row: 8, col: 3 });
    board.place_digit(9, Cell { row: 8, col: 5 });
    println!("{}", format(&board).unwrap());
}
