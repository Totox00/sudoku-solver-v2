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
type Anyhow = Option<()>;

impl Board {
    pub fn solve(&mut self) {
        self.place_hidden_single();
        self.clean_pairs();
    }

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

    pub fn clean_row(&mut self, row: usize, ignore: &[usize], val: u16) -> bool {
        let mut has_changed = false;
        for i in 0..self.size {
            if ignore.contains(&i) {
                continue;
            }
            if let Some(true) = self.clean_cell(Cell { row, col: i }, val) {
                has_changed = true;
            }
        }
        has_changed
    }

    pub fn clean_col(&mut self, col: usize, ignore: &[usize], val: u16) -> bool {
        let mut has_changed = false;
        for i in 0..self.size {
            if ignore.contains(&i) {
                continue;
            }
            if let Some(true) = self.clean_cell(Cell { row: i, col }, val) {
                has_changed = true;
            }
        }
        has_changed
    }

    pub fn clean_reg(&mut self, cell: Cell, ignore: &[Cell], val: u16) -> bool {
        let mut has_changed = false;
        let mut regions = vec![];
        for region in self.regions.iter().filter(|reg| reg.contains(&cell)) {
            regions.push(region.clone());
        }
        for region in regions {
            for cell in region {
                if ignore.contains(&cell) {
                    continue;
                }
                if let Some(true) = self.clean_cell(cell, val) {
                    has_changed = true;
                }
            }
        }
        has_changed
    }

    pub fn clean_cell(&mut self, cell: Cell, val: u16) -> Option<bool> {
        let mut has_changed = false;
        let mut last_val = None;
        let cell_val = self.get_mut_cell(&cell)?;
        assert!(*cell_val != 1 << val, "Cell has no possibilities");
        if *cell_val & (1 << val) > 0 {
            has_changed = true;
            *cell_val &= !(1 << val);
            #[allow(clippy::cast_possible_truncation)]
            if cell_val.count_ones() == 1 {
                last_val = Some(cell_val.trailing_zeros() as u16);
            }
        }
        if let Some(new_val) = last_val {
            self.place_digit(new_val, cell);
        }
        Some(has_changed)
    }

    pub fn place_hidden_single(&mut self) {
        let size = self.size;
        for cell in (0..size).flat_map(|row| (0..size).map(move |col| Cell { row, col })) {
            if self.place_if_hidden_single(cell).is_some() {
                return self.solve();
            }
        }
    }

    pub fn place_if_hidden_single(&mut self, cell: Cell) -> Anyhow {
        if self.get_cell(&cell)?.count_ones() == 1 {
            return None;
        }
        let single = {
            let cell_val = self.get_cell(&cell)?;
            let in_row = self.get_row_nums(cell.row, &[cell.col])?;
            let possible =
                (0..self.size).find(|val| cell_val & (1 << val) > 0 && in_row & (1 << val) == 0);
            if possible.is_some() {
                possible
            } else {
                let in_col = self.get_col_nums(cell.col, &[cell.row])?;
                let possible = (0..self.size)
                    .find(|val| cell_val & (1 << val) > 0 && in_col & (1 << val) == 0);
                if possible.is_some() {
                    possible
                } else {
                    let in_reg = self.get_reg_nums(cell, &[cell])?;
                    let possible = (0..self.size)
                        .find(|val| cell_val & (1 << val) > 0 && in_reg & (1 << val) == 0);
                    if possible.is_some() {
                        possible
                    } else {
                        None
                    }
                }
            }
        };
        #[allow(clippy::cast_possible_truncation)]
        if let Some(val) = single {
            self.place_digit(val as u16, cell);
            Some(())
        } else {
            None
        }
    }

    pub fn get_row_nums(&self, row: usize, ignore: &[usize]) -> Option<u16> {
        let mut digits = 0;
        for col in 0..9 {
            if ignore.contains(&col) {
                continue;
            }
            digits |= self.get_cell_coords(row, col)?;
        }
        Some(digits)
    }

    pub fn get_col_nums(&self, col: usize, ignore: &[usize]) -> Option<u16> {
        let mut digits = 0;
        for row in 0..9 {
            if ignore.contains(&row) {
                continue;
            }
            digits |= self.get_cell_coords(row, col)?;
        }
        Some(digits)
    }

    pub fn get_reg_nums(&self, cell: Cell, ignore: &[Cell]) -> Option<u16> {
        let mut digits = 0;
        let mut regions = vec![];
        for region in self.regions.iter().filter(|reg| reg.contains(&cell)) {
            regions.push(region.clone());
        }
        for region in regions {
            for cell in region {
                if ignore.contains(&cell) {
                    continue;
                }
                digits |= self.get_cell(&cell)?;
            }
        }

        Some(digits)
    }

    pub fn clean_pairs(&mut self) {
        let size = self.size;
        let possible_pair_cells: Vec<_> = (0..size)
            .flat_map(|row| (0..size).map(move |col| Cell { row, col }))
            .filter(|cell| self.get_cell(cell).unwrap_or(0).count_ones() == 2)
            .collect();

        let pairs: Vec<_> = possible_pair_cells[..]
            .iter()
            .zip(0..)
            .flat_map(|(a, i)| possible_pair_cells[i..].iter().map(move |b| (a, b)))
            .filter(|(a, b)| a != b)
            .filter(|(a, b)| self.get_cell(a) == self.get_cell(b))
            .filter(|(a, b)| {
                a.row == b.row
                    || a.col == b.col
                    || self
                        .regions
                        .iter()
                        .filter(|region| region.contains(a))
                        .any(|areg| {
                            self.regions
                                .iter()
                                .filter(|region| region.contains(b))
                                .any(move |breg| areg == breg)
                        })
            })
            .collect();

        let mut has_changed = false;
        for (a, b) in pairs {
            #[allow(clippy::cast_possible_truncation)]
            let val1 = self.get_cell(a).unwrap_or(1).trailing_zeros() as u16;
            #[allow(clippy::cast_possible_truncation)]
            let val2 =
                (self.get_cell(a).unwrap_or(1) >> (val1 + 1)).trailing_zeros() as u16 + val1 + 1;

            if a.row == b.row {
                has_changed = self.clean_row(a.row, &[a.col, b.col], val1) || has_changed;
                has_changed = self.clean_row(a.row, &[a.col, b.col], val2) || has_changed;
            } else if a.col == b.col {
                has_changed = self.clean_col(a.col, &[a.row, b.row], val1) || has_changed;
                has_changed = self.clean_col(a.col, &[a.row, b.row], val2) || has_changed;
            } else {
                has_changed = self.clean_reg(*a, &[*a, *b], val1) || has_changed;
                has_changed = self.clean_reg(*a, &[*a, *b], val2) || has_changed;
            }
        }

        if has_changed {
            return self.solve();
        }
    }
}
