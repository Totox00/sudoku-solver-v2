use std::borrow::BorrowMut;

use crate::defaults::{default_cell, default_region_bounds, default_regions};

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

    pub fn to_string(self) -> String {
        let (rwidth, rheight) = default_region_bounds(self.size);
        let mut out = String::new();
        out.push('╔');
        for i in 0..rheight {
            if i > 0 {
                out.push('╦');
            }
            for j in 0..rwidth {
                if j > 0 {
                    out.push('╤');
                }
                out.push_str("═".repeat(rwidth).as_str());
            }
        }
        out.push('╗');

        for region_row in 0..rwidth {
            if region_row > 0 {
                out.push_str("\n╠");
                for i in 0..rheight {
                    if i > 0 {
                        out.push('╬');
                    }
                    for j in 0..rwidth {
                        if j > 0 {
                            out.push('╪');
                        }
                        out.push_str("═".repeat(rwidth).as_str());
                    }
                }
                out.push('╣');
            }
            for cell_row in 0..rheight {
                if cell_row > 0 {
                    out.push_str("\n╟");
                    for i in 0..rheight {
                        if i > 0 {
                            out.push('╫');
                        }
                        for j in 0..rwidth {
                            if j > 0 {
                                out.push('┼');
                            }
                            out.push_str("─".repeat(rwidth).as_str());
                        }
                    }
                    out.push('╢');
                }
                for digit_row in 0..rheight {
                    out.push_str("\n║");
                    for region_col in 0..rheight {
                        if region_col > 0 {
                            out.push('║');
                        }
                        for cell_col in 0..rwidth {
                            if cell_col > 0 {
                                out.push('│');
                            }
                            for digit_col in 1..=rwidth {
                                out.push(
                                    if self
                                        .get_cell_coords(
                                            cell_row + region_row * rheight,
                                            cell_col + region_col * rwidth,
                                        )
                                        .unwrap()
                                        & 1 << digit_col + digit_row * rwidth
                                        > 0
                                    {
                                        char::from_digit(
                                            (digit_col + digit_row * rwidth) as u32,
                                            10,
                                        )
                                        .unwrap_or(' ')
                                    } else {
                                        ' '
                                    },
                                )
                            }
                        }
                    }
                    out.push('║')
                }
            }
        }

        out.push_str("\n╚");
        for i in 0..rheight {
            if i > 0 {
                out.push('╩');
            }
            for j in 0..rwidth {
                if j > 0 {
                    out.push('╧');
                }
                out.push_str("═".repeat(rwidth).as_str());
            }
        }
        out.push('╝');

        out
    }
}
