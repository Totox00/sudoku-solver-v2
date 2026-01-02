use crate::{
    board::{Board, Cell},
    defaults::default_cell,
    SIZE,
};

#[derive(Debug, Clone)]
pub struct Group<const S: usize> {
    pub cells: [Cell; S],
    pub vals: u16,
}

pub fn from_board2(board: &Board) -> Vec<Group<2>> {
    (0..SIZE)
        .flat_map(|row| (0..(SIZE - 1)).flat_map(move |col_a| ((col_a + 1)..SIZE).map(move |col_b| [Cell { row, col: col_a }, Cell { row, col: col_b }])))
        .filter(|cells| board.no_known(cells))
        .filter_map(|cells| {
            let mut other_values = 0;
            for col in 0..SIZE {
                if col != cells[0].col && col != cells[1].col {
                    other_values |= board.cells[cells[0].row][col];
                }
            }
            let vals = !other_values & default_cell();
            if vals.count_ones() == 2 {
                Some(Group { cells, vals })
            } else {
                None
            }
        })
        .chain(
            (0..SIZE)
                .flat_map(|col| (0..(SIZE - 1)).flat_map(move |row_a| ((row_a + 1)..SIZE).map(move |row_b| [Cell { row: row_a, col }, Cell { row: row_b, col }])))
                .filter(|cells| board.no_known(cells))
                .filter_map(|cells| {
                    let mut other_values = 0;
                    for row in 0..SIZE {
                        if row != cells[0].row && row != cells[1].row {
                            other_values |= board.cells[row][cells[0].col];
                        }
                    }
                    let vals = !other_values & default_cell();
                    if vals.count_ones() == 2 {
                        Some(Group { cells, vals })
                    } else {
                        None
                    }
                }),
        )
        .chain(board.regions.iter().flat_map(|region| {
            (0..(region.len() - 1))
                .flat_map(move |cell_a| ((cell_a + 1)..region.len()).map(move |cell_b| (cell_a, cell_b)))
                .filter(|(cell_a, cell_b)| board.no_known(&[region[*cell_a], region[*cell_b]]))
                .filter_map(|(cell_a, cell_b)| {
                    let mut other_values = 0;
                    for cell in 0..region.len() {
                        if cell != cell_a && cell != cell_b {
                            other_values |= board[region[cell]];
                        }
                    }
                    let vals = !other_values & default_cell();
                    if vals.count_ones() == 2 {
                        Some(Group {
                            cells: [region[cell_a], region[cell_b]],
                            vals,
                        })
                    } else {
                        None
                    }
                })
        }))
        .collect()
}

pub fn from_board3(board: &Board) -> Vec<Group<3>> {
    (0..SIZE)
        .flat_map(|row| {
            (0..(SIZE - 2)).flat_map(move |col_a| {
                ((col_a + 1)..(SIZE - 1)).flat_map(move |col_b| ((col_b + 1)..SIZE).map(move |col_c| [Cell { row, col: col_a }, Cell { row, col: col_b }, Cell { row, col: col_c }]))
            })
        })
        .filter(|cells| board.no_known(cells))
        .filter_map(|cells| {
            let mut other_values = 0;
            for col in 0..SIZE {
                if col != cells[0].col && col != cells[1].col && col != cells[2].col {
                    other_values |= board.cells[cells[0].row][col];
                }
            }
            let vals = !other_values & default_cell();
            if vals.count_ones() == 3 {
                Some(Group { cells, vals })
            } else {
                None
            }
        })
        .chain(
            (0..SIZE)
                .flat_map(|col| {
                    (0..(SIZE - 2)).flat_map(move |row_a| {
                        ((row_a + 1)..(SIZE - 1)).flat_map(move |row_b| ((row_b + 1)..SIZE).map(move |row_c| [Cell { row: row_a, col }, Cell { row: row_b, col }, Cell { row: row_c, col }]))
                    })
                })
                .filter(|cells| board.no_known(cells))
                .filter_map(|cells| {
                    let mut other_values = 0;
                    for row in 0..SIZE {
                        if row != cells[0].row && row != cells[1].row && row != cells[2].row {
                            other_values |= board.cells[row][cells[0].col];
                        }
                    }
                    let vals = !other_values & default_cell();
                    if vals.count_ones() == 3 {
                        Some(Group { cells, vals })
                    } else {
                        None
                    }
                }),
        )
        .chain(board.regions.iter().flat_map(|region| {
            (0..(region.len() - 2))
                .flat_map(move |cell_a| ((cell_a + 1)..(region.len() - 1)).flat_map(move |cell_b| ((cell_b + 1)..region.len()).map(move |cell_c| (cell_a, cell_b, cell_c))))
                .filter(|(cell_a, cell_b, cell_c)| board.no_known(&[region[*cell_a], region[*cell_b], region[*cell_c]]))
                .filter_map(|(cell_a, cell_b, cell_c)| {
                    let mut other_values = 0;
                    for cell in 0..region.len() {
                        if cell != cell_a && cell != cell_b && cell != cell_c {
                            other_values |= board[region[cell]];
                        }
                    }
                    let vals = !other_values & default_cell();
                    if vals.count_ones() == 3 {
                        Some(Group {
                            cells: [region[cell_a], region[cell_b], region[cell_c]],
                            vals,
                        })
                    } else {
                        None
                    }
                })
        }))
        .collect()
}

pub fn from_board4(board: &Board) -> Vec<Group<4>> {
    (0..SIZE)
        .flat_map(|row| {
            (0..(SIZE - 3)).flat_map(move |col_a| {
                ((col_a + 1)..(SIZE - 2)).flat_map(move |col_b| {
                    ((col_b + 1)..(SIZE - 1))
                        .flat_map(move |col_c| ((col_c + 1)..SIZE).map(move |col_d| [Cell { row, col: col_a }, Cell { row, col: col_b }, Cell { row, col: col_c }, Cell { row, col: col_d }]))
                })
            })
        })
        .filter(|cells| board.no_known(cells))
        .filter_map(|cells| {
            let mut other_values = 0;
            for col in 0..SIZE {
                if col != cells[0].col && col != cells[1].col && col != cells[2].col && col != cells[3].col {
                    other_values |= board.cells[cells[0].row][col];
                }
            }
            let vals = !other_values & default_cell();
            if vals.count_ones() == 4 {
                Some(Group { cells, vals })
            } else {
                None
            }
        })
        .chain(
            (0..SIZE)
                .flat_map(|col| {
                    (0..(SIZE - 3)).flat_map(move |row_a| {
                        ((row_a + 1)..(SIZE - 2)).flat_map(move |row_b| {
                            ((row_b + 1)..(SIZE - 1))
                                .flat_map(move |row_c| ((row_c + 1)..SIZE).map(move |row_d| [Cell { row: row_a, col }, Cell { row: row_b, col }, Cell { row: row_c, col }, Cell { row: row_d, col }]))
                        })
                    })
                })
                .filter(|cells| board.no_known(cells))
                .filter_map(|cells| {
                    let mut other_values = 0;
                    for row in 0..SIZE {
                        if row != cells[0].row && row != cells[1].row && row != cells[2].row && row != cells[3].row {
                            other_values |= board.cells[row][cells[0].col];
                        }
                    }
                    let vals = !other_values & default_cell();
                    if vals.count_ones() == 4 {
                        Some(Group { cells, vals })
                    } else {
                        None
                    }
                }),
        )
        .chain(board.regions.iter().flat_map(|region| {
            (0..(region.len() - 3))
                .flat_map(move |cell_a| {
                    ((cell_a + 1)..(region.len() - 2))
                        .flat_map(move |cell_b| ((cell_b + 1)..(region.len() - 1)).flat_map(move |cell_c| ((cell_c + 1)..SIZE).map(move |cell_d| (cell_a, cell_b, cell_c, cell_d))))
                })
                .filter(|(cell_a, cell_b, cell_c, cell_d)| board.no_known(&[region[*cell_a], region[*cell_b], region[*cell_c], region[*cell_d]]))
                .filter_map(|(cell_a, cell_b, cell_c, cell_d)| {
                    let mut other_values = 0;
                    for cell in 0..region.len() {
                        if cell != cell_a && cell != cell_b && cell != cell_c && cell != cell_d {
                            other_values |= board[region[cell]];
                        }
                    }
                    let vals = !other_values & default_cell();
                    if vals.count_ones() == 4 {
                        Some(Group {
                            cells: [region[cell_a], region[cell_b], region[cell_c], region[cell_d]],
                            vals,
                        })
                    } else {
                        None
                    }
                })
        }))
        .collect()
}

impl Board {
    fn no_known(&self, cells: &[Cell]) -> bool {
        cells.iter().all(|cell| !self[*cell].is_power_of_two())
    }
}
