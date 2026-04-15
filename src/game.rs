use rand::{rng, seq::IndexedRandom};

const COLS: usize = 10;
const ROWS: usize = 16;

pub struct TetrisGame<const W: usize = COLS, const H: usize = ROWS> {
    bricks: [[Option<BrickColor>; W]; H],
    current: Tetromino,
}

type BrickColor = (u8, u8, u8);
type Position = (i32, i32);

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
    Z,
}

const I_SHAPE: [Position; 4] = [(0, 0), (0, 0), (0, 0), (0, 0)];
const J_SHAPE: [Position; 4] = [(0, 0), (0, 0), (0, 0), (0, 0)];
const L_SHAPE: [Position; 4] = [(0, 0), (0, 0), (0, 0), (0, 0)];
const N_SHAPE: [Position; 4] = [(0, 0), (0, 0), (0, 0), (0, 0)];
const O_SHAPE: [Position; 4] = [(0, 0), (0, 0), (0, 0), (0, 0)];
const T_SHAPE: [Position; 4] = [(0, 0), (0, 0), (0, 0), (0, 0)];
const Z_SHAPE: [Position; 4] = [(0, 0), (0, 0), (0, 0), (0, 0)];

impl Tetromino {
    fn new(kind: TetrominoKind, color: BrickColor) -> Self {
        Self {
            position: (3, -4),
            kind,
            color,
        }
    }

    fn random() -> Self {
        use TetrominoKind::*;
        const KINDS: [TetrominoKind; 7] = [I, J, L, N, O, T, Z];
        let mut rng = rng();
        let kind = *KINDS.choose(&mut rng).unwrap();
        Self::new(kind, (0, 0, 0))
    }
}
impl TetrisGame {
    pub fn new() -> Self {
        Self {
            bricks: [[None; COLS]; ROWS],
            current: Tetromino::random(),
        }
    }
}
