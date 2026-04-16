use std::{
    io::{self, Stdout},
    process::exit,
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, StyledContent, Stylize},
};

use crate::game::{Position, TetrisGame};

impl TetrisGame {
    pub fn update(&mut self, delta: Duration) {
        self.time += delta;
        if self.time > Duration::from_millis(500) {
            self.step();
            self.time = Duration::ZERO;
        }
    }

    pub fn draw(&self, stdout: &mut Stdout) -> io::Result<()> {
        for (y, row) in self.bricks.iter().enumerate() {
            for (x, brick) in row.iter().enumerate() {
                if let &Some((r, g, b)) = brick {
                    queue!(
                        stdout,
                        MoveTo(y as u16, x as u16),
                        Print("#".with(Color::Rgb { r, g, b }))
                    )?;
                } else {
                    queue!(
                        stdout,
                        MoveTo(y as u16, x as u16),
                        Print(".".with(Color::White))
                    )?;
                }
            }
        }
        let (r, g, b) = self.current.color;
        for Position { x, y } in self.current.get_bricks() {
            queue!(
                stdout,
                MoveTo(y as u16, x as u16),
                Print("#".with(Color::Rgb { r, g, b }))
            )?;
        }
        Ok(())
    }
}
