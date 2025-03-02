use std::io::{stdout, Write};

use anyhow::Context;
use crossterm::{
    cursor::{MoveTo, Show},
    queue,
    style::Print,
    terminal::{self, Clear, ClearType},
};

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> anyhow::Result<()> {
        terminal::enable_raw_mode().context("couldn't enable raw mode")?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)?;
        Self::execute()?;

        Ok(())
    }

    pub fn terminate() -> anyhow::Result<()> {
        Self::execute()?;
        terminal::disable_raw_mode().context("couldn't disable raw mode")?;
        Ok(())
    }

    pub fn clear_screen() -> anyhow::Result<()> {
        queue!(stdout(), Clear(ClearType::All)).context("failed to clean the screen!")?;

        Ok(())
    }

    pub fn show_cursor() -> anyhow::Result<()> {
        queue!(stdout(), Show).context("failed to show cursor")?;

        Ok(())
    }

    pub fn print(txt: &str) -> anyhow::Result<()> {
        queue!(stdout(), Print(txt))?;

        Ok(())
    }

    pub fn move_cursor_to(x: u16, y: u16) -> anyhow::Result<()> {
        queue!(stdout(), MoveTo(x, y)).context("failed to move the cursor")?;
        Ok(())
    }

    pub fn size() -> anyhow::Result<(u16, u16)> {
        terminal::size().context("failed to get terminal size")
    }

    pub fn execute() -> anyhow::Result<()> {
        stdout().flush().context("failed to flush the queue")?;
        Ok(())
    }
}
