use wasm_bindgen::prelude::*;

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
    fn go(mut self, direction: Direction) -> Self {
        match direction {
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Up => self.y -= 1,
        };
        self
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

    fn pos_to_num(&self, pos: Position) -> Option<usize> {
        let Position { y, x } = pos;
        self.inner
            .get(y as usize)
            .and_then(|r| r.get(x as usize))
            .copied()
    }

    fn bad(pos: Position, field: &Vec<Vec<usize>>) -> bool {
        let Position { y, x } = pos;
        !matches!(
            field.get(y as usize).and_then(|r| r.get(x as usize)),
            Some(0)
        )
    }

    fn new(n: usize) -> Self {
        let mut inner = vec![vec![0; n]; n];
        let mut lookup = Vec::with_capacity(n * n);
        let mut pos = Position::new(0, 0);
        let mut direction = Direction::Right;

        for c in (1..=(n * n)).rev() {
            inner[pos.y as usize][pos.x as usize] = c;
            lookup.push(pos);
            // Do a dummy move see if it gets us out of bounds
            // if not that is the move else make a turn and do the move
            let mut new_pos = pos.go(direction);
            if Self::bad(new_pos, &inner) {
                direction = direction.turn();
                new_pos = pos.go(direction);
            }
            pos = new_pos;
        }
        Self { inner, lookup }
    }
}

struct BitMap {
    inner: Vec<u32>,
    ptrs: Vec<usize>,
}

impl BitMap {
    fn new(n: usize, p: usize) -> Self {
        Self {
            inner: vec![0; n + 1],
            ptrs: vec![1; p],
        }
    }

    fn next(&mut self, p: usize) -> Option<usize> {
        let mask = 1u32 << p;
        while self.ptrs[p] < self.inner.len()
            && !(self.inner[self.ptrs[p]] == mask || self.inner[self.ptrs[p]] == 0)
        {
            self.ptrs[p] += 1;
        }

        if self.ptrs[p] < self.inner.len() {
            self.inner[self.ptrs[p]] = u32::MAX;
            Some(self.ptrs[p])
        } else {
            None
        }
    }

    fn remove(&mut self, n: usize, p: usize) {
        self.inner[n] |= 1 << p;
    }

    fn empty(&self) -> bool {
        self.ptrs.iter().all(|&p| p == self.inner.len())
    }
}

struct Game {
    field: Field,
    inner: Vec<Vec<i32>>,
    map: BitMap,
    moves: Vec<Vec<(isize, isize)>>,
}

impl Game {
    fn new(n: usize, moves: Vec<Vec<(isize, isize)>>) -> Self {
        Self {
            field: Field::new(n),
            inner: vec![vec![-1; n]; n],
            map: BitMap::new(n * n, moves.len()),
            moves,
        }
    }

    fn step(&mut self) {
        while !self.map.empty() {
            for p in 0..self.moves.len() {
                if let Some(cell_num) = self.map.next(p) {
                    let pos = self.field.num_to_pos(cell_num).unwrap();
                    self.inner[pos.y as usize][pos.x as usize] = p as i32;

                    for a_cell in self
                        .apply(pos, &self.moves[p])
                        .iter()
                        .filter_map(|&p| self.field.pos_to_num(p))
                    {
                        self.map.remove(a_cell, p);
                    }
                }
            }
        }
    }

    fn apply(&self, pos: Position, moves: &[(isize, isize)]) -> Vec<Position> {
        let Position { y, x } = pos;
        moves
            .iter()
            .map(|&(dy, dx)| Position::new(y + dy, x + dx))
            .collect()
    }
}

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
pub fn generate(size: usize, flat: &[i32], lengths: &[u32]) -> GridResult {
    let mut moves = Vec::with_capacity(lengths.len());
    let mut start = 0;
    for &len in lengths {
        let piece = &flat[start..start + len as usize];
        moves.push(parse_offsets(piece));
        start += len as usize;
    }

    let mut game = Game::new(size, moves);
    game.step();
    GridResult {
        size,
        data: game.inner.into_iter().flatten().collect(),
    }
}

mod test {

    use super::*;

    #[test]
    fn test_t() {
        let mut game = Game::new(
            10,
            vec![
                parse_offsets(&[-2, -1, -2, 1, -1, 2, 1, 2, 2, 1, 2, -1, 1, -2, -1, -2]),
                parse_offsets(&[-2, -1, -2, 1, -1, 2, 1, 2, 2, 1, 2, -1, 1, -2, -1, -2]),
                parse_offsets(&[-2, -1, -2, 1, -1, 2, 1, 2, 2, 1, 2, -1, 1, -2, -1, -2]),
            ],
        );
        game.step();

        for r in game.inner {
            for v in r {
                print!("{}", v);
            }
            println!("");
        }
    }
}
