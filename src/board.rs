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
type Anyhow<T = ()> = Option<T>;

impl Board {
    pub fn new_custom_regions(size: usize, regions: Vec<Region>) -> Self {
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

    pub fn place_digit(&mut self, val: u16, cell: Cell) {
        *self.get_mut_cell(&cell).unwrap() = 1 << val;

        self.clean_col(cell.col, &[cell.row], val);
        self.clean_row(cell.row, &[cell.col], val);
        self.clean_reg(cell, &[cell], val);
    }

    pub fn clean_row(&mut self, row: usize, ignore: &[usize], val: u16) {
        for i in 0..self.size {
            if ignore.contains(&i) {
                continue;
            }
            self.clean_cell(Cell { row, col: i }, val);
        }
    }

    pub fn clean_col(&mut self, col: usize, ignore: &[usize], val: u16) {
        for i in 0..self.size {
            if ignore.contains(&i) {
                continue;
            }
            self.clean_cell(Cell { row: i, col }, val);
        }
    }

    pub fn clean_reg(&mut self, cell: Cell, ignore: &[Cell], val: u16) {
        for region in self
            .regions
            .clone()
            .iter()
            .filter(|reg| reg.contains(&cell))
        {
            for cell in region {
                if ignore.contains(cell) {
                    continue;
                }
                self.clean_cell(*cell, val);
            }
        }
    }

    pub fn clean_cell(&mut self, cell: Cell, val: u16) -> Anyhow {
        let mut last_val = None;
        let cell_val = self.get_mut_cell(&cell)?;
        assert!(*cell_val != 1 << val, "Cell has no possibilities");
        if *cell_val & (1 << val) > 0 {
            *cell_val &= !(1 << val);
            #[allow(clippy::cast_possible_truncation)]
            if cell_val.count_ones() == 1 {
                last_val = Some(cell_val.trailing_zeros() as u16);
            }
        }
        if let Some(new_val) = last_val {
            self.place_digit(new_val, cell);
        }
        Some(())
    }
}
