#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            Direction::Right => Self::Down,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
            Direction::Up => Self::Right,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
struct Position {
    pub y: isize,
    pub x: isize,
}

impl Position {
    // FIXME: This can be implemented more nicely.
    fn go(self, direction: &Direction) -> Self {
        let Position { y, x } = self;

        match direction {
            Direction::Right => Position::new(y, x + 1),
            Direction::Down => Position::new(y + 1, x),
            Direction::Left => Position::new(y, x - 1),
            Direction::Up => Position::new(y - 1, x),
        }
    }

    fn new(y: isize, x: isize) -> Self {
        Self { y, x }
    }
}

struct Field {
    inner: Vec<Vec<usize>>,
    lookup: Vec<Position>,
}

impl Field {
    fn display(&self) {
        for row in &self.inner {
            println!("{:?}", row);
        }
    }

    fn num_to_pos(&self, num: usize) -> Option<Position> {
        let len = self.lookup.len();
        self.lookup.get(len - num).copied()
    }

    fn pos_to_num(&self, pos: &Position) -> Option<usize> {
        let Position { y, x } = pos;
        let y = *y as usize;
        let x = *x as usize;
        self.inner.get(y).and_then(|s| s.get(x)).copied()
    }

    fn bad(pos: Position, field: &Vec<Vec<usize>>) -> bool {
        let Position { y, x } = pos;
        y < 0
            || y as usize >= field.len()
            || x < 0
            || x as usize >= field[0].len()
            || field[y as usize][x as usize] != 0
    }

    fn new(n: usize) -> Self {
        let mut inner = vec![vec![0; n]; n];
        let mut lookup = Vec::with_capacity(n * n);
        let mut pos = Position { y: 0, x: 0 };
        let mut direction = Direction::Right;

        for c in (1..=(n * n)).rev() {
            inner[pos.y as usize][pos.x as usize] = c;
            lookup.push(pos);
            // Do a dummy move see if it gets us out of bounds
            // if not that is the move else make a turn and do the move
            let mut new_pos = pos.go(&direction);
            if Self::bad(new_pos, &inner) {
                direction = direction.turn();
                new_pos = pos.go(&direction);
            }
            pos = new_pos;
        }
        Self { inner, lookup }
    }
}

struct BitMap {
    inner: Vec<bool>,
    ptr: usize,
}

impl BitMap {
    fn new(n: usize) -> Self {
        Self {
            inner: vec![true; n + 1],
            ptr: 1,
        }
    }

    fn pop_first(&mut self) -> Option<usize> {
        while self.ptr < self.inner.len() && !self.inner[self.ptr] {
            self.ptr += 1;
        }
        if self.ptr < self.inner.len() {
            self.inner[self.ptr] = false;
            Some(self.ptr)
        } else {
            None
        }
    }

    fn remove(&mut self, n: &usize) {
        self.inner[*n] = false;
    }
}

struct Game {
    field: Field,
    // FIXME: Have a game board?
    inner: Vec<Vec<i32>>,
    a_pq: BitMap,
    b_pq: BitMap,
    move_a: Vec<(isize, isize)>,
    move_b: Vec<(isize, isize)>,
}

impl Game {
    fn new(n: usize, move_a: Vec<(isize, isize)>, move_b: Vec<(isize, isize)>) -> Self {
        Self {
            field: Field::new(n),
            inner: vec![vec![0; n]; n],
            a_pq: BitMap::new(n * n),
            b_pq: BitMap::new(n * n),
            move_a,
            move_b,
        }
    }

    fn step(&mut self) {
        while let Some(v) = self.a_pq.pop_first() {
            // Mark the position
            let Position { y, x } = self.field.num_to_pos(v).unwrap();
            self.inner[y as usize][x as usize] = 1;

            //  Remove all the now invalid fields
            self.b_pq.remove(&v);
            for cp in self
                .apply(self.field.num_to_pos(v).unwrap(), &self.move_a)
                .iter()
                .filter_map(|p| self.field.pos_to_num(p))
            {
                self.b_pq.remove(&cp);
            }

            if let Some(v) = self.b_pq.pop_first() {
                let Position { y, x } = self.field.num_to_pos(v).unwrap();
                self.inner[y as usize][x as usize] = 2;

                self.a_pq.remove(&v);
                for cp in self
                    .apply(self.field.num_to_pos(v).unwrap(), &self.move_b)
                    .iter()
                    .filter_map(|p| self.field.pos_to_num(p))
                {
                    self.a_pq.remove(&cp);
                }
            }
        }

        // Could be that there are still moves for B left
        while let Some(v) = self.b_pq.pop_first() {
            let Position { y, x } = self.field.num_to_pos(v).unwrap();
            self.inner[y as usize][x as usize] = 2;

            self.a_pq.remove(&v);
            for cp in self
                .apply(self.field.num_to_pos(v).unwrap(), &self.move_b)
                .iter()
                .filter_map(|p| self.field.pos_to_num(p))
            {
                self.a_pq.remove(&cp);
            }
        }
    }

    fn apply(&self, pos: Position, moves: &[(isize, isize)]) -> Vec<Position> {
        moves
            .iter()
            .map(|&(dy, dx)| Position {
                y: pos.y + dy,
                x: pos.x + dx,
            })
            .collect()
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GridResult {
    pub size: usize,
    data: Vec<i32>, // flattened row-major
}

#[wasm_bindgen]
impl GridResult {
    pub fn data_ptr(&self) -> *const i32 {
        self.data.as_ptr()
    }
    pub fn data_len(&self) -> usize {
        self.data.len()
    }
}

fn parse_offsets(flat: &[i32]) -> Vec<(isize, isize)> {
    flat.chunks_exact(2)
        .map(|c| (c[0] as isize, c[1] as isize))
        .collect()
}

#[wasm_bindgen]
pub fn generate(size: usize, offsets_a: &[i32], offsets_b: &[i32]) -> GridResult {
    let moves_a = parse_offsets(offsets_a);
    let moves_b = parse_offsets(offsets_b);
    let mut game = Game::new(size, moves_a, moves_b);
    game.step();

    GridResult {
        size,
        data: game.inner.into_iter().flatten().collect(),
    }
}
