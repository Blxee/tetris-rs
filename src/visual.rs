use std::io::{self, Stdout};

use crossterm::{queue, style::Print};

use crate::game::TetrisGame;

impl TetrisGame {
    pub fn draw(&self, stdout: &mut Stdout) -> io::Result<()> {
        queue!(stdout, Print("board"))?;
        Ok(())
    }
}
