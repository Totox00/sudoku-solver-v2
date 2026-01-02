use std::{
    ops::{Add, AddAssign},
    rc::Rc,
};

use crate::{
    board::{get_regions_with_cells, Board, Cell},
    misc::cells,
};

#[derive(Debug, Clone)]
pub struct Group<const S: usize> {
    pub relation: Relation,
    pub cells: [Cell; S],
    pub vals: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct Relation {
    pub row: bool,
    pub col: bool,
    pub reg: bool,
}

macro_rules! init_group {
    ($groups:ident) => {
        $groups.iter().zip(1..)
    };
}

macro_rules! finish_group {
    ($iter:expr, $groups:ident, $board:ident, $size:expr) => {
        $iter
            .flat_map(|(a, i)| $groups[i..].iter().map(move |b| a.add(b)))
            .filter(|group| group.vals.count_ones() <= $size)
            .map(|group| group.calc_relations($board))
            .filter(|group| group.relation.col || group.relation.row || group.relation.reg)
    };
}

macro_rules! expand_group {
    ($iter:expr, $groups:ident) => {
        $iter
            .flat_map(|(a, i)| $groups[i..].iter().map(move |b| (a.add(b), i + 1)))
            .filter(|(group, _)| group.vals.count_ones() <= 4)
    };
}

pub fn from_board2(board: &Board) -> Rc<[Group<2>]> {
    let groups: Vec<_> = cells()
        .iter()
        .filter_map(|cell| {
            let vals = board[*cell];
            if vals.count_ones() > 1 {
                Some(Group {
                    relation: Relation { row: true, col: true, reg: true },
                    cells: [*cell],
                    vals,
                })
            } else {
                None
            }
        })
        .collect();

    finish_group!(init_group!(groups), groups, board, 2).filter(Group::no_repeats).collect()
}

pub fn from_board3(board: &Board) -> Rc<[Group<3>]> {
    let groups: Vec<_> = cells()
        .iter()
        .filter_map(|cell| {
            let vals = board[*cell];
            if vals.count_ones() > 1 {
                Some(Group {
                    relation: Relation { row: true, col: true, reg: true },
                    cells: [*cell],
                    vals,
                })
            } else {
                None
            }
        })
        .collect();

    finish_group!(expand_group!(init_group!(groups), groups), groups, board, 3).filter(Group::no_repeats).collect()
}

pub fn from_board4(board: &Board) -> Rc<[Group<4>]> {
    let groups: Vec<_> = cells()
        .iter()
        .filter_map(|cell| {
            let vals = board[*cell];
            if vals.count_ones() > 1 {
                Some(Group {
                    relation: Relation { row: true, col: true, reg: true },
                    cells: [*cell],
                    vals,
                })
            } else {
                None
            }
        })
        .collect();

    finish_group!(expand_group!(expand_group!(init_group!(groups), groups), groups), groups, board, 4)
        .filter(Group::no_repeats)
        .collect()
}

impl<const S: usize> Group<{ S }> {
    pub fn calc_relations(self, board: &Board) -> Self {
        let mut regs = get_regions_with_cells!(board, self.cells);
        Self {
            relation: Relation {
                row: self.cells.iter().map(|cell| cell.row).all(|row| row == self.cells[0].row),
                col: self.cells.iter().map(|cell| cell.col).all(|col| col == self.cells[0].col),
                reg: regs.next().is_some(),
            },
            cells: self.cells,
            vals: self.vals,
        }
    }

    pub fn no_repeats(&self) -> bool {
        self.cells.iter().zip(1..).all(|(c, i)| !self.cells[i..].contains(c))
    }

    fn add(&self, rhs: &Group<1>) -> Group<{ S + 1 }> {
        let mut cells = [Cell::default(); S + 1];
        cells[..S].copy_from_slice(&self.cells);
        cells[S..].copy_from_slice(&rhs.cells);

        Group {
            relation: self.relation + rhs.relation,
            cells,
            vals: self.vals | rhs.vals,
        }
    }
}

impl Add for Relation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Relation {
            row: self.row && rhs.row,
            col: self.col && rhs.col,
            reg: self.reg && rhs.reg,
        }
    }
}

impl AddAssign for Relation {
    fn add_assign(&mut self, rhs: Self) {
        *self = Relation {
            row: self.row && rhs.row,
            col: self.col && rhs.col,
            reg: self.reg && rhs.reg,
        }
    }
}
