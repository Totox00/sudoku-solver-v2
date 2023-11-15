use crate::{
    board::{get_regions_with_cells, Board, Cell},
    misc::{cells, is_set},
};

#[derive(Debug, Clone)]
struct Colouring<'a> {
    nodes: Vec<ColourNode>,
    board: &'a Board,
}

#[derive(Debug, Clone)]
struct ColourNode {
    cell: Cell,
    possible: u16,
    state: u16,
    connects_to: Vec<(usize, Option<u16>)>,
}

#[derive(Debug, Clone)]
pub struct ColourMap {
    pub eliminated: Vec<(Cell, u16)>,
    pub placed: Vec<(Cell, u16)>,
}

pub fn from_board(board: &Board) -> ColourMap {
    let mut colouring = Colouring::new(board);
    let cells = cells(board);

    for cell in cells.iter() {
        if let Some(val) = board.get_cell(cell) {
            if val.count_ones() == 2 {
                colouring.add_cell(*cell, val);
            }
        }
    }

    for pair in cells
        .iter()
        .zip(1..)
        .flat_map(|(a, i)| cells[i..].iter().map(move |b| (a, b)))
    {
        if let (Some(a), Some(b)) = (board.get_cell(pair.0), board.get_cell(pair.1)) {
            let mut is_added = 0;
            if pair.0.row == pair.1.row {
                let mut others = 0;
                for col in 0..board.size {
                    if col != pair.0.col && col != pair.1.col {
                        others |= board.get_cell_coords(pair.0.row, col).unwrap();
                    }
                }
                let overlap = a & b;
                for d in 1..=board.size {
                    if overlap & 1 << d > 0 && others & 1 << d == 0 {
                        #[allow(clippy::cast_possible_truncation)]
                        colouring.add_pair(*pair.0, *pair.1, d as u16);
                        is_added |= 1 << d;
                    }
                }
            } else if pair.0.col == pair.1.col {
                let mut others = 0;
                for row in 0..board.size {
                    if row != pair.0.row && row != pair.1.row {
                        others |= board.get_cell_coords(row, pair.0.col).unwrap();
                    }
                }
                let overlap = a & b;
                for d in 1..=board.size {
                    if is_added & 1 << d == 0 && overlap & 1 << d > 0 && others & 1 << d == 0 {
                        #[allow(clippy::cast_possible_truncation)]
                        colouring.add_pair(*pair.0, *pair.1, d as u16);
                        is_added |= 1 << d;
                    }
                }
            }
            for reg in get_regions_with_cells!(board, &[pair.0, pair.1]) {
                let mut others = 0;
                for cell in reg {
                    if cell != pair.0 && cell != pair.1 {
                        others |= board.get_cell(cell).unwrap();
                    }
                }
                let overlap = a & b;
                for d in 1..=board.size {
                    if is_added & 1 << d == 0 && overlap & 1 << d > 0 && others & 1 << d == 0 {
                        #[allow(clippy::cast_possible_truncation)]
                        colouring.add_pair(*pair.0, *pair.1, d as u16);
                        is_added |= 1 << d;
                    }
                }
            }
        }
    }

    colouring.dedup();
    colouring.connect();

    let possible = colouring.get_possible_colourings();

    dbg!(&possible.len());

    colouring.clear_state();

    ColourMap {
        eliminated: vec![],
        placed: vec![],
    }
}

impl Colouring<'_> {
    fn new(board: &Board) -> Colouring<'_> {
        Colouring {
            nodes: vec![],
            board,
        }
    }

    fn add_cell(&mut self, cell: Cell, possible: u16) {
        self.nodes.push(ColourNode {
            cell,
            possible,
            state: possible,
            connects_to: vec![],
        });
    }

    fn add_pair(&mut self, cell_a: Cell, cell_b: Cell, value: u16) {
        let possible_a = self.board.get_cell(&cell_a);
        let possible_b = self.board.get_cell(&cell_b);
        let len = self.nodes.len();

        if let (Some(pa), Some(pb)) = (possible_a, possible_b) {
            self.nodes.push(ColourNode {
                cell: cell_a,
                possible: pa,
                state: pa,
                connects_to: vec![(len + 1, Some(value))],
            });

            self.nodes.push(ColourNode {
                cell: cell_b,
                possible: pb,
                state: pb,
                connects_to: vec![(len, Some(value))],
            });
        }
    }

    fn dedup(&mut self) {
        let mut new: Vec<(Vec<usize>, ColourNode)> = vec![];

        for i in 0..self.nodes.len() {
            let node = self.nodes.get_mut(i).unwrap();
            if let Some(same_i) = new.iter().position(|(_, n)| n.cell == node.cell) {
                let same = new.get_mut(same_i).unwrap();

                same.1.connects_to.append(&mut node.connects_to);
                same.0.push(i);
            } else {
                new.push((vec![i], node.clone()));
            }
        }

        self.nodes = vec![];
        for (_, node) in &new {
            let new_connections: Vec<_> = node
                .connects_to
                .iter()
                .map(|(i, connection)| {
                    (
                        new.iter()
                            .position(|(old_i, _)| old_i.contains(&i))
                            .unwrap(),
                        *connection,
                    )
                })
                .collect();

            self.nodes.push(ColourNode {
                cell: node.cell,
                possible: node.possible,
                state: node.state,
                connects_to: new_connections,
            });
        }
    }

    fn connect(&mut self) {
        let pairs: Vec<_> = (0..self.nodes.len())
            .zip(1..)
            .flat_map(|(a, i)| (i..self.nodes.len()).map(move |b| (a, b)))
            .collect();
        for (n1, n2) in pairs {
            if self.nodes[n1]
                .cell
                .can_see(self.board, &self.nodes[n2].cell)
                && self.nodes[n1].possible & self.nodes[n2].possible > 0
            {
                self.nodes.get_mut(n1).unwrap().connects_to.push((n2, None));
                self.nodes.get_mut(n2).unwrap().connects_to.push((n1, None));
            }
        }
    }

    fn clear_state(&mut self) {
        for node in &mut self.nodes {
            node.state = node.possible;
        }
    }

    fn colour(&mut self, node: usize, val: u16) -> bool {
        let cascade = if let Some(node) = self.nodes.get_mut(node) {
            if node.state.count_ones() > 1 {
                node.state = 1 << val;
                node.connects_to.clone()
            } else {
                return node.state == 1 << val;
            }
        } else {
            return false;
        };

        cascade.iter().all(|(other, maybe_pair)| {
            if let Some(p_val) = maybe_pair {
                if val == *p_val {
                    true
                } else {
                    self.colour(*other, *p_val)
                }
            } else {
                let other_node = self.nodes.get_mut(*other).unwrap();
                if other_node.state.count_ones() == 2 && is_set!(other_node.state, val) {
                    #[allow(clippy::cast_possible_truncation)]
                    let new_state = (other_node.state & !(1 << val)).trailing_zeros() as u16;
                    self.colour(*other, new_state)
                } else {
                    other_node.state &= !(1 << val);
                    true
                }
            }
        })
    }

    fn get_possible_colourings(&self) -> Vec<Vec<(Cell, u16)>> {
        let mut out = vec![];

        if let Some(first) = self
            .nodes
            .iter()
            .position(|node| node.state.count_ones() > 1)
        {
            let mut tmp;
            let pair = get_possible_pair(self.nodes[first].state);

            tmp = self.clone();
            if tmp.colour(first, pair.0) {
                out.append(&mut tmp.get_possible_colourings());
            }

            tmp = self.clone();
            if tmp.colour(first, pair.1) {
                out.append(&mut tmp.get_possible_colourings());
            }
        } else {
            out.push(self.placed_digits());
        }

        out
    }

    fn placed_digits(&self) -> Vec<(Cell, u16)> {
        #[allow(clippy::cast_possible_truncation)]
        self.nodes
            .iter()
            .filter(|n| n.state.count_ones() == 1)
            .map(|n| (n.cell, n.state.trailing_zeros() as u16))
            .collect()
    }
}

#[allow(clippy::cast_possible_truncation)]
fn get_possible_pair(val: u16) -> (u16, u16) {
    let val1 = val.trailing_zeros() as u16;
    (val1, (val >> (val1 + 1)).trailing_zeros() as u16 + val1 + 1)
}
