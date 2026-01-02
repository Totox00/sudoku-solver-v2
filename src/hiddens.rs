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
        .collect()
}

impl Board {
    fn no_known(&self, cells: &[Cell]) -> bool {
        cells.iter().all(|cell| !self[*cell].is_power_of_two())
    }
}
