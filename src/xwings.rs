use std::rc::Rc;

use crate::{board::Board, defaults::default_cell, misc::is_set};

#[derive(Debug, Clone, Copy)]
pub struct XWing {
    pub clear_rows: bool,
    pub rows: (usize, usize),
    pub cols: (usize, usize),
    pub val: u16,
}

pub fn from_board(board: &Board) -> Rc<[XWing]> {
    let size = board.size;

    let pairs: Vec<_> = (0..size)
        .zip(1..)
        .flat_map(|(a, i)| (i..size).map(move |b| (a, b)))
        .collect();

    pairs[..]
        .iter()
        .flat_map(|a| pairs[..].iter().map(move |b| (a, b)))
        .flat_map(|(rows, cols)| {
            [
                XWing {
                    clear_rows: false,
                    rows: *rows,
                    cols: *cols,
                    val: 0,
                },
                XWing {
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
                    board.get_cell_coords(xwing.rows.0, xwing.cols.0).unwrap(),
                    board.get_cell_coords(xwing.rows.1, xwing.cols.0).unwrap(),
                    board.get_cell_coords(xwing.rows.0, xwing.cols.1).unwrap(),
                    board.get_cell_coords(xwing.rows.1, xwing.cols.1).unwrap(),
                ],
            )
        })
        .filter(|(_xwing, vals)| vals.iter().all(|v| v.count_ones() > 1))
        .flat_map(|(xwing, vals)| {
            let v = vals.iter().fold(default_cell(size), |acc, val| acc & val);
            (1..=size).filter_map(move |d| {
                if is_set!(v, d) {
                    #[allow(clippy::cast_possible_truncation)]
                    Some(XWing {
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
                (0..size).all(|row| {
                    xwing.rows.0 == row
                        || xwing.rows.1 == row
                        || !is_set!(board.get_cell_coords(row, xwing.cols.0).unwrap(), xwing.val)
                }) && (0..size).all(|row| {
                    xwing.rows.0 == row
                        || xwing.rows.1 == row
                        || !is_set!(board.get_cell_coords(row, xwing.cols.1).unwrap(), xwing.val)
                })
            } else {
                (0..size).all(|col| {
                    xwing.cols.0 == col
                        || xwing.cols.1 == col
                        || !is_set!(board.get_cell_coords(xwing.rows.0, col).unwrap(), xwing.val)
                }) && (0..size).all(|col| {
                    xwing.cols.0 == col
                        || xwing.cols.1 == col
                        || !is_set!(board.get_cell_coords(xwing.rows.1, col).unwrap(), xwing.val)
                })
            }
        })
        .collect()
}
