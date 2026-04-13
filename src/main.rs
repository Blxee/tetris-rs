use std::{
    io::{Result, Stdout, stdout},
    thread::sleep,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{self, Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, poll, read},
    execute,
    style::Print,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, size},
};

fn main() -> Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, Hide)?;
    let (cols, rows) = size()?;
    let program_start = Instant::now();
    let mut last_frame_time = program_start;

    loop {
        let delta = last_frame_time.elapsed();
        if matches!(poll(Duration::from_secs(0)), Ok(true)) {
            if let Event::Key(key) = read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        _ => (),
                    }
                }
            }
        }
        // do whatevee the hell you want here ;b
        execute!(
            stdout,
            MoveTo(cols / 3, rows / 2),
            Print(format!("delta: {}", delta.as_secs_f32()))
        )?;

        sleep(Duration::from_secs_f32(1. / 60.));
        last_frame_time = Instant::now();
    }

    execute!(stdout, Show)?;
    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
