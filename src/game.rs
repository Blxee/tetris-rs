pub struct TetrisGame<const W: usize = 10, const H: usize = 16> {
    tetrominos: [[Tetromino; W]; H],
    current: Tetromino,
}

struct Tetromino;

impl TetrisGame {
    pub fn new() -> Self {
        Self {
            tetrominos: Vec::new(),
            current: Tetromino,
        }
    }
}
