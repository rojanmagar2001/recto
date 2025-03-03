use buffer::Buffer;

use super::terminal::{Size, Terminal};

mod buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn render() -> anyhow::Result<()> {
        let Size { height, .. } = Terminal::size()?;

        for current_row in 0..height {
            Terminal::clear_line()?;

            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }

            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    pub fn draw_empty_row() -> anyhow::Result<()> {
        Terminal::print("~")?;

        Ok(())
    }

    fn draw_welcome_message() -> anyhow::Result<()> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");

        let width = Terminal::size()?.width as usize;

        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding);

        welcome_message = format!("~{spaces}{welcome_message}");

        welcome_message.truncate(width);

        Terminal::print(&welcome_message)?;

        Ok(())
    }
}
