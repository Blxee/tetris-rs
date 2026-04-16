use std::{
    ops::{Add, Index, IndexMut},
    time::Duration,
};

use rand::{rng, seq::IndexedRandom};

use crate::pos;

const COLS: usize = 10;
const ROWS: usize = 16;

pub struct TetrisGame<const W: usize = COLS, const H: usize = ROWS> {
    pub bricks: [[Option<BrickColor>; W]; H],
    pub current: Tetromino,
    pub time: Duration,
}

type BrickColor = (u8, u8, u8);
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// #[derive(Clone, Copy)]
pub struct Tetromino {
    position: Position,
    kind: TetrominoKind,
    pub color: BrickColor,
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

    pub fn get_bricks(&self) -> [Position; 4] {
        let bricks = match self.kind {
            TetrominoKind::I => I_SHAPE,
            TetrominoKind::J => J_SHAPE,
            TetrominoKind::L => L_SHAPE,
            TetrominoKind::N => N_SHAPE,
            TetrominoKind::O => O_SHAPE,
            TetrominoKind::T => T_SHAPE,
            TetrominoKind::S => S_SHAPE,
        };
        bricks.map(|pos| pos + pos!(0, 1))
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

    pub fn get_size(&self) -> (usize, usize) {
        (self.bricks.len(), self.bricks[0].len())
    }

    pub fn step(&mut self) {
        self.current.position.x += 1;
        for brick in self.current.get_bricks() {
            let lower_brick = self[brick + pos!(0, 1)];
            if lower_brick.is_none() {
                self.fix_current_bricks();
                self.current = Tetromino::random();
            }
        }
    }

    fn fix_current_bricks(&mut self) {
        for brick in self.current.get_bricks() {
            self[brick] = Some(self.current.color);
        }
    }
}

impl Index<Position> for TetrisGame {
    type Output = Option<BrickColor>;

    fn index(&self, index: Position) -> &Self::Output {
        &self.bricks[index.x as usize][index.y as usize]
    }
}

impl IndexMut<Position> for TetrisGame {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.bricks[index.x as usize][index.y as usize]
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        pos!(self.x + rhs.x, self.y + self.y)
    }
}

#[macro_export]
macro_rules! pos {
    ($x: expr, $y: expr) => {
        Position { x: $x, y: $y }
    };
}
