use std::{
    fmt::Display,
    io::{stdout, Write},
};

use anyhow::Context;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{self, Clear, ClearType},
    Command,
};

pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> anyhow::Result<()> {
        terminal::enable_raw_mode().context("couldn't enable raw mode")?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position::new(0, 0))?;
        Self::execute()?;

        Ok(())
    }

    pub fn terminate() -> anyhow::Result<()> {
        Self::execute()?;
        terminal::disable_raw_mode().context("couldn't disable raw mode")?;
        Ok(())
    }

    pub fn clear_screen() -> anyhow::Result<()> {
        Self::queue_command(Clear(ClearType::All)).context("failed to clear the screen!")?;

        Ok(())
    }

    pub fn clear_line() -> anyhow::Result<()> {
        Self::queue_command(Clear(ClearType::CurrentLine))
            .context("failed to clear current line")?;

        Ok(())
    }

    pub fn show_cursor() -> anyhow::Result<()> {
        Self::queue_command(Show).context("failed to show cursor")?;

        Ok(())
    }

    pub fn hide_cursor() -> anyhow::Result<()> {
        Self::queue_command(Hide).context("failed to hide cursor")?;

        Ok(())
    }

    pub fn print<T: Display>(txt: T) -> anyhow::Result<()> {
        queue!(stdout(), Print(txt))?;

        Ok(())
    }

    pub fn move_cursor_to(position: Position) -> anyhow::Result<()> {
        Self::queue_command(MoveTo(position.x, position.y)).context("failed to move the cursor")?;
        Ok(())
    }

    pub fn size() -> anyhow::Result<Size> {
        let size = terminal::size().context("failed to get terminal size")?;

        Ok(Size {
            width: size.0,
            height: size.1,
        })
    }

    pub fn execute() -> anyhow::Result<()> {
        stdout().flush().context("failed to flush the queue")?;
        Ok(())
    }

    pub fn queue_command<T: Command>(command: T) -> anyhow::Result<()> {
        queue!(stdout(), command)?;

        Ok(())
    }
}
