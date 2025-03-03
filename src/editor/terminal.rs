use std::io::{stdout, Write};

use anyhow::Context;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{self, Clear, ClearType},
    Command,
};

pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Position {
    pub fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }
}

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> anyhow::Result<()> {
        terminal::enable_raw_mode().context("couldn't enable raw mode")?;
        Self::clear_screen()?;
        Self::move_caret_to(Position::default())?;
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

    pub fn show_caret() -> anyhow::Result<()> {
        Self::queue_command(Show).context("failed to show cursor")?;

        Ok(())
    }

    pub fn hide_caret() -> anyhow::Result<()> {
        Self::queue_command(Hide).context("failed to hide cursor")?;

        Ok(())
    }

    pub fn print(txt: &str) -> anyhow::Result<()> {
        queue!(stdout(), Print(txt))?;

        Ok(())
    }

    pub fn move_caret_to(position: Position) -> anyhow::Result<()> {
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))
            .context("failed to move the cursor")?;
        Ok(())
    }

    pub fn size() -> anyhow::Result<Size> {
        let size = terminal::size().context("failed to get terminal size")?;

        Ok(Size {
            width: size.0 as usize,
            height: size.1 as usize,
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
