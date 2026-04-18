use std::{
    collections::HashSet,
    io::{self, Stdout},
    process::exit,
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, StyledContent, Stylize},
};

use crate::{
    game::{Position, TetrisGame},
    pos,
};

impl TetrisGame {
    pub fn update(&mut self, delta: Duration) {
        self.time += delta;
        if self.time > Duration::from_millis(500) {
            self.step();
            self.time = Duration::ZERO;
        }
    }

    pub fn draw(&self, stdout: &mut Stdout) -> io::Result<()> {
        let current = HashSet::from(self.current.get_bricks());
        for (y, row) in self.bricks.iter().enumerate() {
            for (x, brick) in row.iter().enumerate() {
                if current.contains(&pos!(x, y)) {
                    let (r, g, b) = self.current.color;
                    queue!(
                        stdout,
                        MoveTo(x as u16, y as u16),
                        Print("#".with(Color::Rgb { r, g, b }))
                    )?;
                } else if let &Some((r, g, b)) = brick {
                    queue!(
                        stdout,
                        MoveTo(x as u16, y as u16),
                        Print("#".with(Color::Rgb { r, g, b }))
                    )?;
                } else {
                    queue!(
                        stdout,
                        MoveTo(x as u16, y as u16),
                        Print(".".with(Color::White))
                    )?;
                }
            }
        }
        Ok(())
    }
}
