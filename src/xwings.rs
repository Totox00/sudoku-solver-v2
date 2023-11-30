use std::rc::Rc;

use crate::{board::Board, defaults::default_cell, misc::is_set, SIZE};

#[derive(Debug, Clone, Copy)]
pub struct XWing2 {
    pub clear_rows: bool,
    pub rows: [usize; 2],
    pub cols: [usize; 2],
    pub val: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct XWing3 {
    pub clear_rows: bool,
    pub rows: [usize; 3],
    pub cols: [usize; 3],
    pub val: u16,
}

pub fn from_board(board: &Board) -> Rc<[XWing2]> {
    let pairs: Vec<_> = (0..SIZE)
        .zip(1..)
        .flat_map(|(a, i)| (i..SIZE).map(move |b| [a, b]))
        .collect();

    pairs[..]
        .iter()
        .flat_map(|a| pairs[..].iter().map(move |b| (a, b)))
        .flat_map(|(rows, cols)| {
            [
                XWing2 {
                    clear_rows: false,
                    rows: *rows,
                    cols: *cols,
                    val: 0,
                },
                XWing2 {
                    clear_rows: true,
                    rows: *rows,
                    cols: *cols,
                    val: 0,
                },
            ]
        })
        .map(|xwing| {
            (
                xwing,
                [
                    board.get_cell_coords(xwing.rows[0], xwing.cols[0]).unwrap(),
                    board.get_cell_coords(xwing.rows[1], xwing.cols[0]).unwrap(),
                    board.get_cell_coords(xwing.rows[0], xwing.cols[1]).unwrap(),
                    board.get_cell_coords(xwing.rows[1], xwing.cols[1]).unwrap(),
                ],
            )
        })
        .filter(|(_xwing, vals)| vals.iter().all(|v| v.count_ones() > 1))
        .flat_map(|(xwing, vals)| {
            let v = vals.iter().fold(default_cell(), |acc, val| acc & val);
            (1..=SIZE).filter_map(move |d| {
                if is_set!(v, d) {
                    #[allow(clippy::cast_possible_truncation)]
                    Some(XWing2 {
                        clear_rows: xwing.clear_rows,
                        rows: xwing.rows,
                        cols: xwing.cols,
                        val: d as u16,
                    })
                } else {
                    None
                }
            })
        })
        .filter(|xwing| {
            if xwing.clear_rows {
                (0..SIZE).all(|row| {
                    xwing.rows[0] == row
                        || xwing.rows[1] == row
                        || !is_set!(
                            board.get_cell_coords(row, xwing.cols[0]).unwrap(),
                            xwing.val
                        )
                }) && (0..SIZE).all(|row| {
                    xwing.rows[0] == row
                        || xwing.rows[1] == row
                        || !is_set!(
                            board.get_cell_coords(row, xwing.cols[1]).unwrap(),
                            xwing.val
                        )
                })
            } else {
                (0..SIZE).all(|col| {
                    xwing.cols[0] == col
                        || xwing.cols[1] == col
                        || !is_set!(
                            board.get_cell_coords(xwing.rows[0], col).unwrap(),
                            xwing.val
                        )
                }) && (0..SIZE).all(|col| {
                    xwing.cols[0] == col
                        || xwing.cols[1] == col
                        || !is_set!(
                            board.get_cell_coords(xwing.rows[1], col).unwrap(),
                            xwing.val
                        )
                })
            }
        })
        .collect()
}
