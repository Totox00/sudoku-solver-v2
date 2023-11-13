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
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum Resolvable {
    Cell((Cell, u16, u16)),
    Pair((Cell, Cell, u16)),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    None,
    A,
    B,
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

    let possible = colouring.get_possible_colourings(board);

    dbg!(&possible);
    dbg!(&possible[0]
        .iter()
        .filter(|a| possible[1..].iter().all(|other| other.contains(a)))
        .collect::<Vec<_>>());

    while let Some(first) = colouring
        .nodes
        .iter()
        .position(|node| node.state == State::None)
    {
        if !colouring.colour(first, State::A, board) {
            return ColourMap {
                eliminated: vec![],
                placed: vec![match colouring.nodes[first].resolvable {
                    Resolvable::Cell((c, _, v)) | Resolvable::Pair((_, c, v)) => (c, v),
                }],
            };
        }
    }

    colouring.clear_state();

    while let Some(first) = colouring
        .nodes
        .iter()
        .position(|node| node.state == State::None)
    {
        if !colouring.colour(first, State::B, board) {
            return ColourMap {
                eliminated: vec![],
                placed: vec![match colouring.nodes[first].resolvable {
                    Resolvable::Cell((c, v, _)) | Resolvable::Pair((c, _, v)) => (c, v),
                }],
            };
        }
    }

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
            state: State::None,
        })
    }

    fn add_pair(&mut self, pair: (Cell, Cell, u16)) {
        self.nodes.push(ColourNode {
            resolvable: Resolvable::Pair(pair),
            connects_to: vec![],
            state: State::None,
        })
    }

    fn connect(&mut self, board: &Board) {
        let pairs: Vec<_> = (0..self.nodes.len())
            .zip(1..)
            .flat_map(|(a, i)| (i..self.nodes.len()).map(move |b| (a, b)))
            .collect();
        for pair in pairs {
            if should_connect(
                &self.nodes[pair.0].resolvable,
                &self.nodes[pair.1].resolvable,
                board,
            ) {
                self.nodes.get_mut(pair.0).unwrap().connects_to.push(pair.1);
                self.nodes.get_mut(pair.1).unwrap().connects_to.push(pair.0);
            }
        }
    }

    fn colour(&mut self, node: usize, state: State, board: &Board) -> bool {
        if state == State::None {
            return true;
        }

        let cascade = if let Some(node) = self.nodes.get_mut(node) {
            match node.state {
                State::None => {
                    node.state = state;
                    node.connects_to.clone()
                }
                _ => return true,
            }
        } else {
            return true;
        };

        cascade.iter().all(|other| {
            let matching_state = matching_state(
                &self.nodes[node].resolvable,
                &self.nodes[*other].resolvable,
                state,
                board,
            );

            if matching_state != State::None {
                self.colour(*other, matching_state, board)
            } else {
                true
            }
        })
    }

    fn clear_state(&mut self) {
        for node in &mut self.nodes {
            node.state = State::None
        }
    }

    fn placed_digits(&self) -> Vec<(Cell, u16)> {
        let mut digits = self
            .nodes
            .iter()
            .filter_map(|node| match (node.resolvable, node.state) {
                (Resolvable::Cell((c, v, _)), State::A)
                | (Resolvable::Cell((c, _, v)), State::B)
                | (Resolvable::Pair((c, _, v)), State::A)
                | (Resolvable::Pair((_, c, v)), State::B) => Some((c, v)),
                (_, State::None) => None,
            })
            .collect::<Vec<_>>();

        digits.sort();
        digits.dedup();

        digits
    }

    fn get_possible_colourings(&self, board: &Board) -> Vec<Vec<(Cell, u16)>> {
        let mut out = vec![];

        if let Some(first) = self.nodes.iter().position(|node| node.state == State::None) {
            let mut tmp;

            tmp = self.clone();
            if tmp.colour(first, State::A, board) {
                out.append(&mut tmp.get_possible_colourings(board));
            }

            tmp = self.clone();
            if tmp.colour(first, State::B, board) {
                out.append(&mut tmp.get_possible_colourings(board));
            }
        } else {
            out.push(self.placed_digits().into());
        }

        out
    }
}

#[inline(always)]
fn should_connect(a: &Resolvable, b: &Resolvable, board: &Board) -> bool {
    match (a, b) {
        (Resolvable::Cell((a, a1, a2)), Resolvable::Cell((b, b1, b2))) => {
            (a1 == b1 || a1 == b2 || a2 == b1 || a2 == b2) && a.can_see(board, &b)
        }
        (Resolvable::Cell((c, c1, c2)), Resolvable::Pair((p1, p2, p)))
        | (Resolvable::Pair((p1, p2, p)), Resolvable::Cell((c, c1, c2))) => {
            c == p1
                || c == p2
                || ((c.can_see(board, &p1) || c.can_see(board, &p2)) && (c1 == p || c2 == p))
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
    }
}

#[inline(always)]
fn matching_state(a: &Resolvable, b: &Resolvable, state: State, board: &Board) -> State {
    match (a, b) {
        (Resolvable::Cell((_, a1, a2)), Resolvable::Cell((_, b1, b2))) => {
            if match_state(state, a1, a2) == match_state(state, b2, b1) {
                state
            } else {
                State::None
            }
        }
        (Resolvable::Cell((c, c1, c2)), Resolvable::Pair((p1, p2, p))) => {
            if c == p1 || c == p2 {
                if match_state(state, c1, c2) == p {
                    invert_state_if(c == p2, state)
                } else {
                    invert_state_if(c == p1, state)
                }
            } else {
                if match_state(state, c1, c2) == p {
                    if c.can_see(board, match_state(state, p2, p1)) {
                        state
                    } else {
                        State::None
                    }
                } else {
                    State::None
                }
            }
        }
        (Resolvable::Pair((p1, p2, p)), Resolvable::Cell((c, c1, c2))) => {
            if c == match_state(state, p1, p2) {
                invert_state_if(match_state(state, c2, c1) == p, state)
            } else if c == match_state(state, p2, p1)
                || match_state(state, p1, p2).can_see(board, &c)
            {
                invert_state_if(match_state(state, c1, c2) == p, state)
            } else {
                State::None
            }
        }
        (Resolvable::Pair((c1, c2, a)), Resolvable::Pair((d1, d2, b))) => {
            if match_state(state, c1, c2) == match_state(state, d1, d2) {
                invert_state_if(a != b, state)
            } else if match_state(state, c1, c2).can_see(board, match_state(state, d2, d1)) {
                state
            } else {
                State::None
            }
        }
    }
}

fn match_state<T>(state: State, v1: T, v2: T) -> T {
    match state {
        State::None => unreachable!(),
        State::A => v1,
        State::B => v2,
    }
}

fn invert_state(state: State) -> State {
    match state {
        State::None => State::None,
        State::A => State::B,
        State::B => State::A,
    }
}

fn invert_state_if(b: bool, state: State) -> State {
    if b {
        invert_state(state)
    } else {
        state
    }
}
