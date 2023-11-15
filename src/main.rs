use std::{env::args, fs::read_to_string, io, path::Path, time::Instant};

use crate::{
    board::{Board, Cell},
    format::format,
};

mod board;
mod colouring;
mod defaults;
mod format;
mod groups;
mod intersections;
mod misc;
mod xwings;
mod ywings;

fn main() {
    for mut board in read_puzzle_file(Path::new(
        args()
            .nth(1)
            .expect("Must pass at least one argument")
            .as_str(),
    ))
    .expect("Error reading puzzle file")
    {
        let start = Instant::now();
        board.solve();
        let elapsed = start.elapsed();
        println!("{}", format(&board).unwrap());
        println!("Elapsed time: {elapsed:?}");
    }
}

fn read_puzzle_file(path: &Path) -> io::Result<Vec<Board>> {
    let raw = read_to_string(path)?;

    Ok(raw
        .split("\n\n")
        .map(|puzzle| {
            let lines: Vec<_> = puzzle.split('\n').collect();
            let mut board = Board::new(
                lines
                    .first()
                    .expect("Puzzle does not contain any lines")
                    .len(),
            );

            for (line, row) in lines.iter().zip(0..) {
                for (val, col) in line
                    .chars()
                    .zip(0..)
                    .filter_map(|(chr, col)| chr.to_digit(16).map(|d| (d, col)))
                {
                    #[allow(clippy::cast_possible_truncation)]
                    board.place_digit(val as u16, Cell { row, col });
                }
            }

            board
        })
        .collect())
}
