use std::rc::Rc;

use crate::{
    board::{Board, Cell},
    misc::is_set,
};

#[derive(Debug)]
pub struct YWing {
    pub origin: Cell,
    pub foci: (Cell, Cell),
    pub target: Cell,
    pub val: u16,
}

pub fn from_board(board: &Board) -> Rc<[YWing]> {
    let size = board.size;

    let cells: Vec<_> = (0..size)
        .flat_map(|row| (0..size).map(move |col| Cell { row, col }))
        .collect();

    let useful_cells: Vec<_> = (0..size)
        .flat_map(|row| {
            (0..size).map(move |col| {
                if let Some(vals) = board.get_cell_coords(row, col) {
                    if vals.count_ones() == 2 {
                        Some(Cell { row, col })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .map(|group| {
            group.unwrap_or(Cell {
                row: size,
                col: size,
            })
        })
        .filter(|cell| cell.row != size)
        .collect();

    useful_cells[..]
        .iter()
        .flat_map(|origin| {
            useful_cells[..]
                .iter()
                .zip(1..)
                .filter(move |(foci, _)| origin.can_see(board, foci))
                .flat_map(|(a, i)| useful_cells[i..].iter().map(move |b| (a, b)))
                .filter(move |(_, foci)| origin.can_see(board, foci))
                .filter_map(|(a, b)| {
                    let foci_vals = (board.get_cell(a).unwrap(), board.get_cell(b).unwrap());
                    if board.get_cell(origin).unwrap() ^ foci_vals.0 ^ foci_vals.1 == 0 {
                        #[allow(clippy::cast_possible_truncation)]
                        Some(YWing {
                            origin: *origin,
                            foci: (*a, *b),
                            target: Cell { row: 0, col: 0 },
                            val: (foci_vals.0 & foci_vals.1).trailing_zeros() as u16,
                        })
                    } else {
                        None
                    }
                })
        })
        .flat_map(|ywing| {
            cells
                .iter()
                .filter(move |cell| {
                    **cell != ywing.target && **cell != ywing.foci.0 && **cell != ywing.foci.1
                })
                .map(move |cell| YWing {
                    origin: ywing.origin,
                    foci: ywing.foci,
                    target: *cell,
                    val: ywing.val,
                })
        })
        .filter(|ywing| is_set!(board.get_cell(&ywing.target).unwrap(), ywing.val))
        .filter(|ywing| {
            ywing.foci.0.can_see(board, &ywing.target) && ywing.foci.1.can_see(board, &ywing.target)
        })
        .collect()
}
