use std::io::Stdout;

use crossterm::{queue, style::Print};

use crate::game::TetrisGame;

impl TetrisGame {
    pub fn draw(&self, stdout: &mut Stdout) {
        queue!(stdout, Print("board"));
    }
}
