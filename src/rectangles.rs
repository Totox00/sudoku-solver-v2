use crate::{
    board::{Board, Cell},
    misc::is_set,
    SIZE,
};

pub fn from_board(board: &Board) -> Option<(Vec<Cell>, u16)> {
    for val in 1..=SIZE {
        'row: for row in 0..SIZE {
            let mut has_val = [0; 2];
            let mut idx = 0;

            for col in 0..SIZE {
                if is_set!(board.cells[row][col], val) {
                    if idx == 2 {
                        continue 'row;
                    }

                    if idx == 1 {
                        has_val[1] = col;
                    } else {
                        has_val[0] = col;
                    }

                    idx += 1;
                }
            }

            if idx != 2 {
                continue;
            }

            for (weak_col, region_col) in has_val.into_iter().zip(has_val.into_iter().rev()) {
                let mut weak_cells = vec![];

                for weak_row in 0..SIZE {
                    if weak_row == row {
                        continue;
                    }

                    if !is_set!(board.cells[weak_row][weak_col], val) {
                        continue;
                    }

                    for region in &board.regions {
                        if region.contains(&Cell { row, col: weak_col }) || region.contains(&Cell { row, col: region_col }) {
                            continue;
                        }

                        let cell = Cell { row: weak_row, col: weak_col };
                        if region.contains(&cell) {
                            continue;
                        }

                        if region
                            .iter()
                            .all(|region_cell| region_cell.row == weak_row || region_cell.col == region_col || !is_set!(board[*region_cell], val))
                        {
                            weak_cells.push(cell);
                        }
                    }
                }

                if !weak_cells.is_empty() {
                    return Some((weak_cells, val as u16));
                }
            }
        }
    }

    for val in 1..=SIZE {
        'col: for col in 0..SIZE {
            let mut has_val = [0; 2];
            let mut idx = 0;

            for row in 0..SIZE {
                if is_set!(board.cells[row][col], val) {
                    if idx == 2 {
                        continue 'col;
                    }

                    if idx == 1 {
                        has_val[1] = row;
                    } else {
                        has_val[0] = row;
                    }

                    idx += 1;
                }
            }

            if idx != 2 {
                continue;
            }

            for (weak_row, region_row) in has_val.into_iter().zip(has_val.into_iter().rev()) {
                let mut weak_cells = vec![];

                for weak_col in 0..SIZE {
                    if weak_col == col {
                        continue;
                    }

                    if !is_set!(board.cells[weak_row][weak_col], val) {
                        continue;
                    }

                    for region in &board.regions {
                        if region.contains(&Cell { row: weak_row, col }) || region.contains(&Cell { row: region_row, col }) {
                            continue;
                        }

                        let cell = Cell { row: weak_row, col: weak_col };
                        if region.contains(&cell) {
                            continue;
                        }

                        if region
                            .iter()
                            .all(|region_cell| region_cell.col == weak_col || region_cell.row == region_row || !is_set!(board[*region_cell], val))
                        {
                            weak_cells.push(cell);
                        }
                    }
                }

                if !weak_cells.is_empty() {
                    return Some((weak_cells, val as u16));
                }
            }
        }
    }

    None
}
