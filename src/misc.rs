use std::rc::Rc;

use crate::board::{Board, Cell, Region};

#[derive(Debug, Clone, Copy)]
pub enum Unit<'a> {
    Row(usize),
    Col(usize),
    Reg(&'a Region),
}

pub fn units(board: &Board) -> Rc<[Unit]> {
    board
        .regions
        .iter()
        .map(Unit::Reg)
        .chain((0..board.size).map(Unit::Row))
        .chain((0..board.size).map(Unit::Col))
        .collect()
}

pub fn cells(board: &Board) -> Rc<[Cell]> {
    (0..board.size)
        .flat_map(|row| (0..board.size).map(move |col| Cell { row, col }))
        .collect()
}

macro_rules! is_set {
    ($val:expr, $bit:expr) => {
        $val & (1 << $bit) > 0
    };
}

pub(crate) use is_set;
