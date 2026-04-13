pub struct TetrisGame {
    tetrominos: Vec<Tetromino>,
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
