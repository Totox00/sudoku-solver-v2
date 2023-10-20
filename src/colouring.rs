use crate::{
    board::{get_regions_with_cells, Board, Cell},
    misc::cells,
};

#[derive(Debug, Clone)]
struct Colouring {
    nodes: Vec<ColourNode>,
}

#[derive(Debug, Clone)]
struct ColourNode {
    resolvable: Resolvable,
    connects_to: Vec<usize>,
    is_coloured: Option<bool>,
}

#[derive(Debug, Clone, Copy)]
enum Resolvable {
    Cell((Cell, u16, u16)),
    Pair((Cell, Cell, u16)),
}

#[derive(Debug, Clone)]
pub struct ColourMap {
    pub eliminated: Vec<(Cell, u16)>,
    pub placed: Vec<(Cell, u16)>,
}

pub fn from_board(board: &Board) -> ColourMap {
    let mut colouring = Colouring::new();
    let cells = cells(board);

    for cell in cells.into_iter() {
        if let Some(val) = board.get_cell(cell) {
            if val.count_ones() == 2 {
                let val1 = val.trailing_zeros();
                colouring.add_cell((
                    *cell,
                    val.trailing_zeros() as u16,
                    ((val >> val1 + 1).trailing_zeros() + val1 + 1) as u16,
                ));
            }
        }
    }

    for pair in cells
        .into_iter()
        .zip(1..)
        .flat_map(|(a, i)| cells[i..].iter().map(move |b| (a, b)))
        .filter(|(a, b)| a.can_see(board, b))
    {
        if let (Some(a), Some(b)) = (board.get_cell(pair.0), board.get_cell(pair.1)) {
            let mut is_added = 0;
            if pair.0.row == pair.1.row {
                let mut others = 0;
                for col in 0..board.size {
                    if col != pair.0.col && col != pair.1.col {
                        others |= board.get_cell_coords(pair.0.row, col).unwrap_or(0)
                    }
                }
                let overlap = a & b;
                for d in 1..=board.size {
                    if overlap & 1 << d > 0 && others & 1 << d == 0 {
                        colouring.add_pair((*pair.0, *pair.1, d as u16));
                        is_added |= 1 << d;
                    }
                }
            } else if pair.0.col == pair.1.col {
                let mut others = 0;
                for row in 0..board.size {
                    if row != pair.0.row && row != pair.1.row {
                        others |= board.get_cell_coords(row, pair.0.col).unwrap_or(0)
                    }
                }
                let overlap = a & b;
                for d in 1..=board.size {
                    if is_added & 1 << d == 0 && overlap & 1 << d > 0 && others & 1 << d == 0 {
                        colouring.add_pair((*pair.0, *pair.1, d as u16));
                        is_added |= 1 << d;
                    }
                }
            }
            for reg in get_regions_with_cells!(board, &[pair.0, pair.1]) {
                let mut others = 0;
                for cell in reg {
                    if cell != pair.0 && cell != pair.1 {
                        others |= board.get_cell(cell).unwrap_or(0)
                    }
                }
                let overlap = a & b;
                for d in 1..=board.size {
                    if is_added & 1 << d == 0 && overlap & 1 << d > 0 && others & 1 << d == 0 {
                        colouring.add_pair((*pair.0, *pair.1, d as u16));
                        is_added |= 1 << d;
                    }
                }
            }
        }
    }

    colouring.connect(board);

    dbg!(colouring.nodes.iter().zip(0..).collect::<Vec<_>>());

    ColourMap {
        eliminated: vec![],
        placed: vec![],
    }
}

impl Colouring {
    fn new() -> Self {
        Colouring { nodes: vec![] }
    }

    fn add_cell(&mut self, cell: (Cell, u16, u16)) {
        self.nodes.push(ColourNode {
            resolvable: Resolvable::Cell(cell),
            connects_to: vec![],
            is_coloured: None,
        })
    }

    fn add_pair(&mut self, pair: (Cell, Cell, u16)) {
        self.nodes.push(ColourNode {
            resolvable: Resolvable::Pair(pair),
            connects_to: vec![],
            is_coloured: None,
        })
    }

    fn connect(&mut self, board: &Board) {
        let pairs: Vec<_> = (0..self.nodes.len())
            .zip(1..)
            .flat_map(|(a, i)| (i..self.nodes.len()).map(move |b| (a, b)))
            .collect();
        for pair in pairs {
            if match (self.nodes[pair.0].resolvable, self.nodes[pair.1].resolvable) {
                (Resolvable::Cell((a, a1, a2)), Resolvable::Cell((b, b1, b2))) => {
                    (a1 == b1 || a1 == b2 || a2 == b1 || a2 == b2) && a.can_see(board, &b)
                }
                (Resolvable::Cell((c, c1, c2)), Resolvable::Pair((p1, p2, p)))
                | (Resolvable::Pair((p1, p2, p)), Resolvable::Cell((c, c1, c2))) => {
                    c == p1
                        || c == p2
                        || ((c.can_see(board, &p1) || c.can_see(board, &p2))
                            && (c1 == p || c2 == p))
                }
                (Resolvable::Pair((c1, c2, a)), Resolvable::Pair((d1, d2, b))) => {
                    c1 == d1
                        || c1 == d2
                        || c2 == d1
                        || c2 == d2
                        || (a == b
                            && (c1.can_see(board, &d1)
                                || c1.can_see(board, &d2)
                                || c2.can_see(board, &d1)
                                || c2.can_see(board, &d2)))
                }
            } {
                self.nodes.get_mut(pair.0).unwrap().connects_to.push(pair.1);
                self.nodes.get_mut(pair.1).unwrap().connects_to.push(pair.0);
            }
        }
    }
}
