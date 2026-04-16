mod game;
mod visual;

use std::{
    io::{Result, Stdout, Write, stdout},
    thread::sleep,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{Hide, Show},
    event::{Event, KeyCode, KeyEventKind, poll, read},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, size},
};

use crate::game::TetrisGame;

fn main() -> Result<()> {
    let mut stdout = stdout();
    prepare_terminal(&mut stdout)?;

    let (cols, rows) = size()?;
    let program_start = Instant::now();
    let mut last_frame_time = program_start;

    let tetris_game = TetrisGame::new();

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

        tetris_game.update(delta);
        tetris_game.draw(&mut stdout)?;

        stdout.flush()?;
        sleep(Duration::from_secs_f32(1. / 60.));
        last_frame_time = Instant::now();
    }

    cleanup_terminal(&mut stdout)?;
    Ok(())
}

fn prepare_terminal(stdout: &mut Stdout) -> Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, Hide)?;
    Ok(())
}

fn cleanup_terminal(stdout: &mut Stdout) -> Result<()> {
    execute!(stdout, Show)?;
    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
