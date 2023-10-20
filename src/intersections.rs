use std::rc::Rc;

use crate::{
    board::{Board, Cell, Region},
    misc::{units, Unit},
};

#[derive(Debug, Clone, Copy)]
pub struct Intersection<'a> {
    origin: Unit<'a>,
    target: Unit<'a>,
    val: u16,
}

#[derive(Debug, Clone)]
pub struct IntersectionTarget {
    pub cells: Region,
    pub val: u16,
}

pub fn from_board(board: &Board) -> Rc<[IntersectionTarget]> {
    let size = board.size;
    let units = units(board);

    units
        .iter()
        .zip(1..)
        .flat_map(|(a, i)| {
            units[i..].iter().map(move |b| Intersection {
                origin: *a,
                target: *b,
                val: 0,
            })
        })
        .filter(|intersection| match intersection.origin {
            Unit::Row(_) if !matches!(intersection.target, Unit::Reg(_)) => false,
            Unit::Col(_) if !matches!(intersection.target, Unit::Reg(_)) => false,
            _ => true,
        })
        .flat_map(|intersection| {
            (1..=size)
                .map(move |val| Intersection {
                    origin: intersection.origin,
                    target: intersection.target,
                    val: val as u16,
                })
                .filter_map(|intersection| intersection.is_valid(board))
        })
        .collect()
}

impl Intersection<'_> {
    fn is_valid(&self, board: &Board) -> Option<IntersectionTarget> {
        let origin_cells: Region = match self.origin {
            Unit::Row(row) => (0..board.size).map(|col| Cell { row, col }).collect(),
            Unit::Col(col) => (0..board.size).map(|row| Cell { row, col }).collect(),
            Unit::Reg(reg) => reg.to_vec(),
        };
        let target_cells: Region = match self.target {
            Unit::Row(row) => (0..board.size).map(|col| Cell { row, col }).collect(),
            Unit::Col(col) => (0..board.size).map(|row| Cell { row, col }).collect(),
            Unit::Reg(reg) => reg.to_vec(),
        };

        let overlap: Vec<_> = origin_cells
            .iter()
            .filter(|cell| target_cells.contains(cell))
            .collect();

        if overlap
            .iter()
            .all(|cell| board.get_cell(cell).unwrap_or(1).count_ones() > 1)
            && origin_cells.iter().all(|cell| {
                overlap.contains(&cell) || board.get_cell(cell).unwrap_or(0) & 1 << self.val == 0
            })
        {
            Some(IntersectionTarget {
                cells: target_cells
                    .iter()
                    .filter(|cell| !overlap.contains(cell))
                    .map(|cell| *cell)
                    .collect(),
                val: self.val,
            })
        } else {
            None
        }
    }
}
