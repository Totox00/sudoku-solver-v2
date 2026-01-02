use std::rc::Rc;

use crate::{
    board::{Board, Cell},
    defaults::default_cell,
    misc::is_set,
    SIZE,
};

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

macro_rules! get_values {
    ($board:ident, $xwing:ident) => {
        $xwing
            .rows
            .iter()
            .flat_map(|row| $xwing.cols.iter().map(|col| Cell { row: *row, col: *col }))
            .map(|cell| $board[cell])
            .collect::<Rc<[_]>>()
    };
}

macro_rules! is_valid {
    ($board:ident, $xwing:ident) => {
        if $xwing.clear_rows {
            (0..SIZE).all(|row| {
                $xwing
                    .cols
                    .iter()
                    .all(|col| $xwing.rows.contains(&row) || !is_set!($board.get_cell_coords(row, *col).unwrap(), $xwing.val))
            })
        } else {
            (0..SIZE).all(|col| {
                $xwing
                    .rows
                    .iter()
                    .all(|row| $xwing.cols.contains(&col) || !is_set!($board.get_cell_coords(*row, col).unwrap(), $xwing.val))
            })
        }
    };
}

pub fn from_board2(board: &Board) -> Rc<[XWing2]> {
    let pairs: Vec<_> = (0..SIZE).zip(1..).flat_map(|(a, i)| (i..SIZE).map(move |b| [a, b])).collect();

    pairs[..]
        .iter()
        .flat_map(|a| pairs[..].iter().map(move |b| (a, b)))
        .flat_map(|(rows, cols)| {
            [true, false].iter().map(|clear_rows| XWing2 {
                clear_rows: *clear_rows,
                rows: *rows,
                cols: *cols,
                val: 0,
            })
        })
        .map(|xwing| (xwing, get_values!(board, xwing)))
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
        .filter(|xwing| is_valid!(board, xwing))
        .collect()
}

pub fn from_board3(board: &Board) -> Rc<[XWing3]> {
    let unit_groups: Vec<_> = (0..SIZE)
        .zip(1..)
        .flat_map(|(a, i)| (i..SIZE).zip((i + 1)..).flat_map(move |(b, j)| (j..SIZE).map(move |c| [a, b, c])))
        .collect();

    unit_groups[..]
        .iter()
        .flat_map(|a| unit_groups[..].iter().map(move |b| (a, b)))
        .flat_map(|(rows, cols)| {
            [true, false].iter().map(|clear_rows| XWing3 {
                clear_rows: *clear_rows,
                rows: *rows,
                cols: *cols,
                val: 0,
            })
        })
        .flat_map(|xwing| {
            (1..=SIZE).map(move |d| {
                #[allow(clippy::cast_possible_truncation)]
                XWing3 {
                    clear_rows: xwing.clear_rows,
                    rows: xwing.rows,
                    cols: xwing.cols,
                    val: d as u16,
                }
            })
        })
        .filter(|xwing| is_valid!(board, xwing))
        .collect()
}
