use buffer::Buffer;
use location::Location;

use super::terminal::{Position, Size, Terminal};

mod buffer;
pub mod line;
pub mod location;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    pub location: Location,
    pub scroll_offset: Location,
}

impl View {
    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.scroll_location_into_view();
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

    pub fn get_postion(&self) -> Position {
        self.location.subtract(&self.scroll_offset).into()
    }

    pub fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { width, height } = self.size;
        let mut offset_changed = false;

        // Scroll vertically
        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        //Scroll horizontally
        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }
        self.needs_redraw = offset_changed;
    }

    fn render_buffer(&self) -> anyhow::Result<()> {
        let Size { height, width } = self.size;

        let top = self.scroll_offset.y;
        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {
                let truncated_line = {
                    let left = self.scroll_offset.x;
                    let right = self.scroll_offset.x.saturating_add(width);

                    line.get(left..right)
                };

                Self::render_line(current_row, &truncated_line)?;
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
            location: Location::default(),
            scroll_offset: Location::default(),
        }
    }
}
