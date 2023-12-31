use std::{
    ops::{Add, AddAssign},
    rc::Rc,
};

use crate::{
    board::{get_regions_with_cells, Board, Cell},
    misc::cells,
};

#[derive(Debug, Clone)]
pub struct Group {
    pub relation: Relation,
    pub cells: Vec<Cell>,
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
            .flat_map(|(a, i)| $groups[i..].iter().map(move |b| a.clone() + b.clone()))
            .filter(|group| group.vals.count_ones() <= $size)
            .map(|group| group.calc_relations($board))
            .filter(|group| group.relation.col || group.relation.row || group.relation.reg)
    };
}

macro_rules! expand_group {
    ($iter:expr, $groups:ident) => {
        $iter
            .flat_map(|(a, i)| {
                $groups[i..]
                    .iter()
                    .map(move |b| (a.clone() + b.clone(), i + 1))
            })
            .filter(|(group, _)| group.vals.count_ones() <= 4)
    };
}

pub fn from_board(board: &Board) -> Rc<[Group]> {
    let groups: Vec<_> = cells()
        .iter()
        .filter_map(|cell| {
            let vals = board[*cell];
            if vals.count_ones() > 1 {
                Some(Group {
                    relation: Relation {
                        row: true,
                        col: true,
                        reg: true,
                    },
                    cells: vec![*cell],
                    vals,
                })
            } else {
                None
            }
        })
        .collect();

    finish_group!(init_group!(groups), groups, board, 2)
        .chain(finish_group!(
            expand_group!(init_group!(groups), groups),
            groups,
            board,
            3
        ))
        .chain(finish_group!(
            expand_group!(expand_group!(init_group!(groups), groups), groups),
            groups,
            board,
            4
        ))
        .filter(Group::no_repeats)
        .collect()
}

impl Group {
    pub fn calc_relations(self, board: &Board) -> Self {
        let mut regs = get_regions_with_cells!(board, self.cells);
        Self {
            relation: Relation {
                row: self
                    .cells
                    .iter()
                    .map(|cell| cell.row)
                    .all(|row| row == self.cells[0].row),
                col: self
                    .cells
                    .iter()
                    .map(|cell| cell.col)
                    .all(|col| col == self.cells[0].col),
                reg: regs.next().is_some(),
            },
            cells: self.cells,
            vals: self.vals,
        }
    }

    pub fn no_repeats(&self) -> bool {
        self.cells
            .iter()
            .zip(1..)
            .all(|(c, i)| !self.cells[i..].contains(c))
    }
}

impl Add for Group {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Group {
            relation: self.relation + rhs.relation,
            cells: vec![self.cells, rhs.cells].concat(),
            vals: self.vals | rhs.vals,
        }
    }
}

impl AddAssign for Group {
    fn add_assign(&mut self, rhs: Self) {
        self.relation += rhs.relation;
        self.cells.extend(rhs.cells);
        self.vals |= rhs.vals;
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
