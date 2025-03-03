use std::io::{stdout, Write};

use anyhow::Context;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    Command,
};

#[derive(Default)]
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
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::execute()?;

        Ok(())
    }

    pub fn terminate() -> anyhow::Result<()> {
        Self::leave_alternate_screen()?;
        Self::show_caret()?;
        Self::execute()?;
        terminal::disable_raw_mode().context("couldn't disable raw mode")?;
        Ok(())
    }

    fn enter_alternate_screen() -> anyhow::Result<()> {
        Self::queue_command(EnterAlternateScreen)?;

        Ok(())
    }

    fn leave_alternate_screen() -> anyhow::Result<()> {
        Self::queue_command(LeaveAlternateScreen)?;

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

    pub fn print_row(row: usize, line_text: &str) -> anyhow::Result<()> {
        Self::move_caret_to(Position::new(0, row))?;
        Self::clear_line()?;
        Self::print(line_text)?;

        Ok(())
    }

    pub fn print(txt: &str) -> anyhow::Result<()> {
        Self::queue_command(Print(txt))?;

        Ok(())
    }

    pub fn move_caret_to(position: Position) -> anyhow::Result<()> {
        Self::queue_command(MoveTo(
            u16::try_from(position.col)?,
            u16::try_from(position.row)?,
        ))
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
