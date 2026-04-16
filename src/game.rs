use std::time::Duration;

use rand::{rng, seq::IndexedRandom};

macro_rules! pos {
    ($x: expr, $y: expr) => {
        Position { x: $x, y: $y }
    };
}

const COLS: usize = 10;
const ROWS: usize = 16;

pub struct TetrisGame<const W: usize = COLS, const H: usize = ROWS> {
    pub bricks: [[Option<BrickColor>; W]; H],
    pub current: Tetromino,
    pub time: Duration,
}

type BrickColor = (u8, u8, u8);
struct Position {
    pub x: i32,
    pub y: i32,
}

// #[derive(Clone, Copy)]
struct Tetromino {
    position: Position,
    kind: TetrominoKind,
    color: BrickColor,
}

#[derive(Clone, Copy)]
enum TetrominoKind {
    I,
    J,
    L,
    N,
    O,
    T,
    S,
}

const I_SHAPE: [Position; 4] = [pos!(1, 0), pos!(1, 1), pos!(1, 2), pos!(1, 3)];
const J_SHAPE: [Position; 4] = [pos!(1, 2), pos!(2, 0), pos!(2, 1), pos!(2, 2)];
const L_SHAPE: [Position; 4] = [pos!(1, 0), pos!(1, 1), pos!(1, 2), pos!(2, 2)];
const N_SHAPE: [Position; 4] = [pos!(1, 1), pos!(2, 1), pos!(2, 2), pos!(3, 2)];
const O_SHAPE: [Position; 4] = [pos!(1, 1), pos!(2, 1), pos!(1, 2), pos!(2, 2)];
const T_SHAPE: [Position; 4] = [pos!(1, 1), pos!(2, 1), pos!(3, 1), pos!(2, 2)];
const S_SHAPE: [Position; 4] = [pos!(1, 2), pos!(2, 2), pos!(2, 1), pos!(3, 1)];

impl Tetromino {
    fn new(kind: TetrominoKind, color: BrickColor) -> Self {
        Self {
            position: pos!(3, -4),
            kind,
            color,
        }
    }

    fn random() -> Self {
        use TetrominoKind::*;
        const KINDS: [TetrominoKind; 7] = [I, J, L, N, O, T, S];
        let mut rng = rng();
        let kind = *KINDS.choose(&mut rng).unwrap();
        Self::new(kind, (0, 0, 0))
    }

    pub fn step(&mut self) {
        todo!()
    }
}
impl TetrisGame {
    pub fn new() -> Self {
        Self {
            bricks: [[None; COLS]; ROWS],
            current: Tetromino::random(),
            time: Duration::ZERO,
        }
    }
}
