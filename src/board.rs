use std::{
    ops::{Index, IndexMut},
    rc::Rc,
};

use crate::{
    colouring,
    defaults::{default_cell, default_regions},
    groups, intersections,
    misc::is_set,
    xwings, ywings, SIZE,
};

#[derive(Debug, Clone)]
pub struct Board {
    pub regions: Vec<Region>,
    pub cells: [[u16; SIZE]; SIZE],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
}

pub type Region = Vec<Cell>;
type Anyhow = Option<()>;

macro_rules! get_regions_with_cell {
    ($board:ident, $cell:expr) => {
        $board
            .regions
            .iter()
            .filter(|region| region.contains($cell))
    };
}

pub(crate) use get_regions_with_cell;

macro_rules! get_regions_with_cells {
    ($board:ident, $cells:expr) => {
        $board
            .regions
            .iter()
            .filter(|region| $cells.iter().all(|cell| region.contains(cell)))
    };
}

pub(crate) use get_regions_with_cells;

impl Board {
    pub fn solve(&mut self) {
        self.place_hidden_single();
        self.clean_groups();
        self.clean_xwings2();
        self.clean_ywings();
        self.clean_intersections();
        self.clean_xwings3();
        self.clean_colouring();
    }

    pub fn new_custom_regions(regions: Vec<Region>) -> Self {
        Board {
            regions,
            cells: [[default_cell(); SIZE]; SIZE],
        }
    }

    pub fn new() -> Self {
        Board {
            regions: default_regions(),
            cells: [[default_cell(); SIZE]; SIZE],
        }
    }

    pub fn get_cell_coords(&self, row: usize, col: usize) -> Option<u16> {
        self.cells.get(row)?.get(col).copied()
    }

    pub fn get_mut_cell_coords(&mut self, row: usize, col: usize) -> Option<&mut u16> {
        self.cells.get_mut(row)?.get_mut(col)
    }

    pub fn place_digit(&mut self, val: u16, cell: Cell) {
        self[cell] = 1 << val;

        self.clean_col(cell.col, &[cell.row], val);
        self.clean_row(cell.row, &[cell.col], val);
        self.clean_reg(cell, &[cell], val);
    }

    pub fn clean_row(&mut self, row: usize, ignore: &[usize], val: u16) -> bool {
        let mut has_changed = false;
        for i in 0..SIZE {
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
        for i in 0..SIZE {
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
        let cell_val = &mut self[cell];
        assert!(*cell_val != 1 << val, "Cell has no possibilities");
        if is_set!(*cell_val, val) {
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
        let size = SIZE;
        for cell in (0..size).flat_map(|row| (0..size).map(move |col| Cell { row, col })) {
            if self.place_if_hidden_single(cell).is_some() {
                return self.solve();
            }
        }
    }

    pub fn place_if_hidden_single(&mut self, cell: Cell) -> Anyhow {
        if self[cell].count_ones() == 1 {
            return None;
        }
        let single = {
            let cell_val = self[cell];
            let in_row = self.get_row_nums(cell.row, &[cell.col])?;
            let possible = (0..SIZE).find(|val| is_set!(cell_val, val) && !is_set!(in_row, val));
            if possible.is_some() {
                possible
            } else {
                let in_col = self.get_col_nums(cell.col, &[cell.row])?;
                let possible =
                    (0..SIZE).find(|val| is_set!(cell_val, val) && !is_set!(in_col, val));
                if possible.is_some() {
                    possible
                } else {
                    let in_reg = self.get_reg_nums(cell, &[cell])?;
                    let possible =
                        (0..SIZE).find(|val| is_set!(cell_val, val) && !is_set!(in_reg, val));
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
        for region in get_regions_with_cell!(self, &cell) {
            for cell in region {
                if ignore.contains(cell) {
                    continue;
                }
                digits |= self[*cell];
            }
        }

        Some(digits)
    }

    pub fn clean_groups(&mut self) {
        let groups = groups::from_board(self);

        let mut has_changed = false;
        for group in groups.iter() {
            for val in (0..=SIZE).filter_map(|d| {
                if is_set!(group.vals, d) {
                    #[allow(clippy::cast_possible_truncation)]
                    Some(d as u16)
                } else {
                    None
                }
            }) {
                if group.relation.row {
                    has_changed = self.clean_row(
                        group.cells[0].row,
                        &group.cells.iter().map(|cell| cell.col).collect::<Rc<[_]>>()[..],
                        val,
                    ) || has_changed;
                } else if group.relation.col {
                    has_changed = self.clean_col(
                        group.cells[0].col,
                        &group.cells.iter().map(|cell| cell.row).collect::<Rc<[_]>>()[..],
                        val,
                    ) || has_changed;
                }
                if group.relation.reg {
                    has_changed = self.clean_reg(group.cells[0], &group.cells, val) || has_changed;
                }
            }
        }

        if has_changed {
            self.solve();
        }
    }

    pub fn clean_xwings2(&mut self) {
        let xwings = xwings::from_board2(self);

        let mut has_changed = false;
        for xwing in xwings.iter() {
            if xwing.clear_rows {
                for row in xwing.rows {
                    has_changed = self.clean_row(row, &xwing.cols, xwing.val) || has_changed;
                }
            } else {
                for col in xwing.cols {
                    has_changed = self.clean_col(col, &xwing.rows, xwing.val) || has_changed;
                }
            }
        }

        if has_changed {
            self.solve();
        }
    }

    pub fn clean_xwings3(&mut self) {
        let xwings = xwings::from_board3(self);

        let mut has_changed = false;
        for xwing in xwings.iter() {
            if xwing.clear_rows {
                for row in xwing.rows {
                    has_changed = self.clean_row(row, &xwing.cols, xwing.val) || has_changed;
                }
            } else {
                for col in xwing.cols {
                    has_changed = self.clean_col(col, &xwing.rows, xwing.val) || has_changed;
                }
            }
        }

        if has_changed {
            self.solve();
        }
    }

    pub fn clean_ywings(&mut self) {
        let ywings = ywings::from_board(self);

        let mut has_changed = false;
        for ywing in ywings.iter() {
            has_changed = self.clean_cell(ywing.target, ywing.val).unwrap_or(false) || has_changed;
        }

        if has_changed {
            self.solve();
        }
    }

    pub fn clean_intersections(&mut self) {
        let intersections = intersections::from_board(self);

        let mut has_changed = false;
        for intersection in intersections.iter() {
            for cell in &intersection.cells {
                has_changed =
                    self.clean_cell(*cell, intersection.val).unwrap_or(false) || has_changed;
            }
        }

        if has_changed {
            self.solve();
        }
    }

    pub fn clean_colouring(&mut self) {
        let colour_map = colouring::from_board(self);

        if colour_map.eliminated.is_empty() && colour_map.placed.is_empty() {
            return;
        }

        for (cell, val) in colour_map.placed {
            self.place_digit(val, cell);
        }

        for (cell, val) in colour_map.eliminated {
            self.clean_cell(cell, val);
        }

        self.solve();
    }
}

impl Cell {
    pub fn can_see(&self, board: &Board, target: &Cell) -> bool {
        self.row == target.row
            || self.col == target.col
            || get_regions_with_cells!(board, &[self, target])
                .next()
                .is_some()
    }
}

impl Index<Cell> for Board {
    type Output = u16;

    fn index(&self, index: Cell) -> &Self::Output {
        &self.cells[index.row][index.col]
    }
}

impl IndexMut<Cell> for Board {
    fn index_mut(&mut self, index: Cell) -> &mut Self::Output {
        &mut self.cells[index.row][index.col]
    }
}
