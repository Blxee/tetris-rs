use std::{
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{queue, style::Print};

use crate::game::TetrisGame;

impl TetrisGame {
    pub fn update(&mut self, delta: Duration) {
        self.time += delta;
    }

    pub fn draw(&self, stdout: &mut Stdout) -> io::Result<()> {
        queue!(stdout, Print("board"))?;
        Ok(())
    }
}
