use std::rc::Rc;

use crate::{
    board::{Board, Cell, Region},
    SIZE,
};

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
        .chain((0..SIZE).map(Unit::Row))
        .chain((0..SIZE).map(Unit::Col))
        .collect()
}

pub fn cells() -> Rc<[Cell]> {
    (0..SIZE)
        .flat_map(|row| (0..SIZE).map(move |col| Cell { row, col }))
        .collect()
}

macro_rules! is_set {
    ($val:expr, $bit:expr) => {
        $val & (1 << $bit) > 0
    };
}

pub(crate) use is_set;
