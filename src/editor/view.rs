use std::io::{stdout, Write};

use buffer::Buffer;

use super::terminal::{Size, Terminal};

mod buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl View {
    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.needs_redraw = true;
    }

    pub fn render_line(at: usize, line_text: &str) -> anyhow::Result<()> {
        Terminal::print_row(at, line_text)?;

        Ok(())
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        if !self.needs_redraw {
            return Ok(());
        }

        let Size { width, height } = self.size;

        if width == 0 || height == 0 {
            return Ok(());
        }

        if self.buffer.is_empty() {
            self.render_welcome_screen()?;
        } else {
            self.render_buffer()?;
        }

        self.needs_redraw = false;

        Ok(())
    }

    fn render_buffer(&self) -> anyhow::Result<()> {
        let Size { height, width } = self.size;

        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row) {
                let truncated_line = if line.len() > width {
                    &line[0..width]
                } else {
                    line
                };

                Self::render_line(current_row, truncated_line)?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }

        Ok(())
    }

    fn render_welcome_screen(&self) -> anyhow::Result<()> {
        let Size { height, .. } = self.size;

        for current_row in 0..height {
            if current_row == height / 3 {
                let message = self.build_welcome_message();

                Self::render_line(current_row, &message)?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }

        Ok(())
    }

    fn build_welcome_message(&self) -> String {
        let Size { width, .. } = self.size;

        if width == 0 {
            return " ".to_string();
        }

        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");

        let len = welcome_message.len();

        if width <= len {
            return "~".to_string();
        }

        let padding = width.saturating_sub(len).saturating_sub(1) / 2;
        let spaces = " ".repeat(padding);

        welcome_message = format!("~{spaces}{welcome_message}");

        welcome_message.truncate(width);

        welcome_message
    }

    pub fn load(&mut self, file_name: &str) -> anyhow::Result<()> {
        self.buffer.load(file_name)?;

        Ok(())
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}
