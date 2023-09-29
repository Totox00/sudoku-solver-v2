use std::borrow::BorrowMut;

use crate::defaults::{default_cell, default_regions};

#[derive(Debug, Clone)]
pub struct Board {
    pub size: usize,
    pub regions: Vec<Region>,
    pub cells: Vec<Vec<u16>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
}

pub type Region = Vec<Cell>;
type Anyhow<T = ()> = Result<T, ()>;

impl Board {
    pub fn _new_custom_regions(size: usize, regions: Vec<Region>) -> Self {
        Board {
            size,
            regions,
            cells: vec![vec![default_cell(size); size]; size],
        }
    }

    pub fn new(size: usize) -> Self {
        Board {
            size,
            regions: default_regions(size),
            cells: vec![vec![default_cell(size); size]; size],
        }
    }

    pub fn get_cell(&self, cell: &Cell) -> Option<u16> {
        self.cells.get(cell.row)?.get(cell.col).copied()
    }

    pub fn get_cell_coords(&self, row: usize, col: usize) -> Option<u16> {
        self.cells.get(row)?.get(col).copied()
    }

    pub fn get_mut_cell(&mut self, cell: &Cell) -> Option<&mut u16> {
        self.cells.get_mut(cell.row)?.get_mut(cell.col)
    }

    pub fn get_mut_cell_coords(&mut self, row: usize, col: usize) -> Option<&mut u16> {
        self.cells.get_mut(row)?.get_mut(col)
    }

    pub fn place_digit(&mut self, val: u16, cell: Cell) -> Anyhow {
        *self.get_mut_cell(&cell).unwrap() = 1 << val;

        self.clean_col(cell.col, &[cell.row], val)?;
        self.clean_row(cell.row, &[cell.col], val)?;
        self.clean_reg(cell, &[cell], val)?;

        Ok(())
    }

    pub fn clean_row(&mut self, row: usize, ignore: &[usize], val: u16) -> Anyhow {
        for i in 0..self.size {
            if ignore.contains(&i) {
                continue;
            } else if self.get_cell_coords(row, i).unwrap() == 1 << val {
                return Err(());
            }
            *self.get_mut_cell_coords(row, i).unwrap() &= !(1 << val);
        }
        Ok(())
    }

    pub fn clean_col(&mut self, col: usize, ignore: &[usize], val: u16) -> Anyhow {
        for i in 0..self.size {
            if ignore.contains(&i) {
                continue;
            } else if self.get_cell_coords(i, col).unwrap() == 1 << val {
                return Err(());
            }
            *self.get_mut_cell_coords(i, col).unwrap() &= !(1 << val);
        }
        Ok(())
    }

    pub fn clean_reg(&mut self, cell: Cell, ignore: &[Cell], val: u16) -> Anyhow {
        let cells: &mut Vec<Vec<u16>> = self.cells.borrow_mut();
        for region in self.regions.iter().filter(|reg| reg.contains(&cell)) {
            for cell in region {
                if ignore.contains(&cell) {
                    continue;
                } else if *cells.get(cell.row).unwrap().get(cell.col).unwrap() == 1 << val {
                    return Err(());
                } else {
                    *cells.get_mut(cell.row).unwrap().get_mut(cell.col).unwrap() &= !(1 << val);
                }
            }
        }
        Ok(())
    }
}
