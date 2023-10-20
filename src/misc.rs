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
        .map(|reg| Unit::Reg(reg))
        .chain((0..board.size).map(|i| Unit::Row(i)))
        .chain((0..board.size).map(|i| Unit::Col(i)))
        .collect()
}

pub fn cells(board: &Board) -> Rc<[Cell]> {
    (0..board.size)
        .flat_map(|row| (0..board.size).map(move |col| Cell { row, col }))
        .collect()
}
