use std::io::{Result, Stdout, stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, read},
    execute,
    style::Print,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, size},
};

fn main() -> Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    let (cols, rows) = size()?;

    execute!(stdout, Print(format!("hello world {cols}, {rows}")))?;

    while let Event::Key(key) = read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => break,
                _ => (),
            }
        }
    }

    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
