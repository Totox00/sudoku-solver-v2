use std::{ops::{Add, AddAssign}, rc::Rc};

use crate::board::{get_regions_with_cells, Board, Cell};

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

pub fn from_board(board: &Board) -> Rc<[Group]> {
    let size = board.size;

    let groups: Vec<_> = (0..size)
        .flat_map(|row| {
            (0..size).map(move |col| {
                if let Some(vals) = board.get_cell_coords(row, col) {
                    if vals.count_ones() > 1 {
                        Some(Group {
                            relation: Relation {
                                row: true,
                                col: true,
                                reg: true,
                            },
                            cells: vec![Cell { row, col }],
                            vals,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .map(|group| {
            group.unwrap_or(Group {
                relation: Relation {
                    row: true,
                    col: true,
                    reg: true,
                },
                cells: vec![],
                vals: 0,
            })
        })
        .filter(|group| !group.cells.is_empty())
        .collect();

    groups
        .iter()
        .zip(1..)
        .flat_map(|(a, i)| groups[i..].iter().map(|b| a.clone() + b.clone()))
        .filter(|group| group.vals.count_ones() == 2)
        .map(|group| group.calc_relations(board))
        .filter(|group| group.relation.col || group.relation.row || group.relation.reg)
        .chain(
            groups
                .iter()
                .zip(1..)
                .flat_map(|(a, i)| {
                    groups[i..]
                        .iter()
                        .map(move |b| (a.clone() + b.clone(), i + 1))
                })
                .filter(|(group, _)| group.vals.count_ones() <= 3)
                .flat_map(|(a, i)| groups[i..].iter().map(move |b| a.clone() + b.clone()))
                .filter(|group| group.vals.count_ones() <= 3)
                .map(|group| group.calc_relations(board))
                .filter(|group| group.relation.col || group.relation.row || group.relation.reg),
        )
        .chain(
            groups
                .iter()
                .zip(1..)
                .flat_map(|(a, i)| {
                    groups[i..]
                        .iter()
                        .map(move |b| (a.clone() + b.clone(), i + 1))
                })
                .filter(|(group, _)| group.vals.count_ones() <= 4)
                .flat_map(|(a, i)| {
                    groups[i..]
                        .iter()
                        .map(move |b| (a.clone() + b.clone(), i + 1))
                })
                .filter(|(group, _)| group.vals.count_ones() <= 4)
                .flat_map(|(a, i)| groups[i..].iter().map(move |b| a.clone() + b.clone()))
                .filter(|group| group.vals.count_ones() <= 4)
                .map(|group| group.calc_relations(board))
                .filter(|group| group.relation.col || group.relation.row || group.relation.reg),
        )
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
