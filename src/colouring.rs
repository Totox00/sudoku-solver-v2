use std::cmp::{max, min};

use crate::{
    board::{get_regions_with_cells, Board, Cell},
    misc::{cells, is_set},
    SIZE,
};

#[derive(Debug, Clone)]
struct Colouring<'a> {
    nodes: Vec<ColourNode>,
    board: &'a Board,
}

#[derive(Debug, Clone)]
struct ColourNode {
    cell: Cell,
    val: u16,
    state: State,
    connects_to: Vec<(usize, bool)>, // if true, this or the connected must be true, else both can be false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    None,
    True,
    False,
}

#[derive(Debug, Clone)]
pub struct ColourMap {
    pub eliminated: Vec<(Cell, u16)>,
    pub placed: Vec<(Cell, u16)>,
}

pub fn from_board(board: &Board) -> ColourMap {
    let mut colouring = Colouring::new(board);
    let cells = cells();

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
                for col in 0..SIZE {
                    if col != pair.0.col && col != pair.1.col {
                        others |= board.get_cell_coords(pair.0.row, col).unwrap();
                    }
                }
                let overlap = a & b;
                for d in 1..=SIZE {
                    if overlap & 1 << d > 0 && others & 1 << d == 0 {
                        #[allow(clippy::cast_possible_truncation)]
                        colouring.add_pair(*pair.0, *pair.1, d as u16);
                        is_added |= 1 << d;
                    }
                }
            } else if pair.0.col == pair.1.col {
                let mut others = 0;
                for row in 0..SIZE {
                    if row != pair.0.row && row != pair.1.row {
                        others |= board.get_cell_coords(row, pair.0.col).unwrap();
                    }
                }
                let overlap = a & b;
                for d in 1..=SIZE {
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
                for d in 1..=SIZE {
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

    let mut placed = vec![];
    for cell in &possible[0] {
        if possible[1..]
            .iter()
            .all(|placement| placement.contains(cell))
        {
            placed.push(*cell);
        }
    }

    let eliminated: Vec<_> = cells
        .iter()
        .flat_map(|c| {
            (0..9)
                .filter(|v| is_set!(board.get_cell(c).unwrap(), v))
                .map(move |v| (*c, v))
        })
        .filter(|(c, v)| {
            possible.iter().all(|possibility| {
                possibility.iter().any(|(other_c, other_v)| {
                    c != other_c && v == other_v && c.can_see(board, other_c)
                })
            })
        })
        .collect();

    ColourMap { eliminated, placed }
}

impl Colouring<'_> {
    fn new(board: &Board) -> Colouring<'_> {
        Colouring {
            nodes: vec![],
            board,
        }
    }

    fn add_cell(&mut self, cell: Cell, possible: u16) {
        let vals = get_possible_pair(possible);
        self.nodes.push(ColourNode {
            cell,
            val: vals.0,
            state: State::None,
            connects_to: vec![],
        });
        self.nodes.push(ColourNode {
            cell,
            val: vals.1,
            state: State::None,
            connects_to: vec![],
        });
    }

    fn add_pair(&mut self, cell_a: Cell, cell_b: Cell, val: u16) {
        let len = self.nodes.len();

        self.nodes.push(ColourNode {
            cell: cell_a,
            val,
            state: State::None,
            connects_to: vec![(len + 1, true)],
        });

        self.nodes.push(ColourNode {
            cell: cell_b,
            val,
            state: State::None,
            connects_to: vec![(len, true)],
        });
    }

    fn dedup(&mut self) {
        let mut new: Vec<(Vec<usize>, ColourNode)> = vec![];

        for i in 0..self.nodes.len() {
            let node = self.nodes.get_mut(i).unwrap();
            if let Some(same_i) = new
                .iter()
                .position(|(_, n)| n.cell == node.cell && n.val == node.val)
            {
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
                        new.iter().position(|(old_i, _)| old_i.contains(i)).unwrap(),
                        *connection,
                    )
                })
                .collect();

            self.nodes.push(ColourNode {
                cell: node.cell,
                val: node.val,
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
        for (i1, i2) in pairs {
            if let Some((n1, n2)) = get_mut_pair(&mut self.nodes, (i1, i2)) {
                if (n1.cell == n2.cell)
                    || (n1.val == n2.val && n1.cell.can_see(self.board, &n2.cell))
                {
                    n1.connects_to.push((i2, false));
                    n2.connects_to.push((i1, false));
                }
            }
        }
    }

    fn colour(&mut self, node: usize, state: State) -> bool {
        if state == State::None {
            return true;
        }
        let cascade = if let Some(node) = self.nodes.get_mut(node) {
            if node.state == State::None {
                node.state = state;
                node.connects_to.clone()
            } else {
                return node.state == state;
            }
        } else {
            return false;
        };

        cascade.iter().all(|(other, is_pair)| {
            if state == State::True {
                self.colour(*other, State::False)
            } else if *is_pair && state == State::False {
                self.colour(*other, State::True)
            } else {
                true
            }
        })
    }

    fn get_possible_colourings(&self) -> Vec<Vec<(Cell, u16)>> {
        let mut out = vec![];

        if let Some(first) = self.nodes.iter().position(|node| node.state == State::None) {
            let mut tmp;

            tmp = self.clone();
            if tmp.colour(first, State::True) {
                out.append(&mut tmp.get_possible_colourings());
            }

            tmp = self.clone();
            if tmp.colour(first, State::False) {
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
            .filter(|n| n.state == State::True)
            .map(|n| (n.cell, n.val))
            .collect()
    }
}

#[allow(clippy::cast_possible_truncation)]
fn get_possible_pair(val: u16) -> (u16, u16) {
    let val1 = val.trailing_zeros() as u16;
    (val1, (val >> (val1 + 1)).trailing_zeros() as u16 + val1 + 1)
}

fn get_mut_pair<T>(slice: &mut [T], index: (usize, usize)) -> Option<(&mut T, &mut T)> {
    if index.0 == index.1 {
        None
    } else {
        let (first, second) = (min(index.0, index.1), max(index.0, index.1));
        let (_, tmp) = slice.split_at_mut(first);
        let (x, rest) = tmp.split_at_mut(1);
        let (_, y) = rest.split_at_mut(second - first - 1);
        Some(if index.0 < index.1 {
            (&mut x[0], &mut y[0])
        } else {
            (&mut y[0], &mut x[0])
        })
    }
}
