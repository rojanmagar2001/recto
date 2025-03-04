use std::cmp::min;

use buffer::Buffer;
use line::Line;

use super::{
    editorcommand::Direction,
    terminal::{Position, Size, Terminal},
};

mod buffer;
pub mod line;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Copy, Default)]
pub struct Location {
    pub grapheme_index: usize,
    pub line_index: usize,
}

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    pub text_location: Location,
    pub scroll_offset: Position,
}

impl View {
    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.scroll_text_location_into_view();
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

    pub fn caret_postion(&self) -> Position {
        self.text_location_to_position()
            .saturating_sub(self.scroll_offset)
    }

    fn text_location_to_position(&self) -> Position {
        let row = self.text_location.line_index;

        let col = self.buffer.lines.get(row).map_or(0, |line| {
            line.width_until(self.text_location.grapheme_index)
        });

        Position { col, row }
    }

    pub fn move_point(&mut self, direction: Direction) -> anyhow::Result<()> {
        let Size { height, .. } = Terminal::size()?;

        match direction {
            Direction::Up => self.move_up(1),
            Direction::Down => self.move_down(1),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::PageUp => self.move_up(height.saturating_sub(1)),
            Direction::PageDown => self.move_down(height.saturating_sub(1)),
            Direction::End => self.move_to_start_of_line(),
            Direction::Home => self.move_to_end_of_line(),
        }

        self.scroll_text_location_into_view();

        Ok(())
    }

    fn move_up(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    fn move_down(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    fn move_right(&mut self) {
        let line_width = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        if self.text_location.grapheme_index < line_width {
            self.text_location.grapheme_index += 1;
        } else {
            self.move_to_start_of_line();
            self.move_down(1);
        }
    }

    fn move_left(&mut self) {
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index -= 1;
        } else {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }

    fn move_to_start_of_line(&mut self) {
        self.text_location.grapheme_index = 0;
    }

    fn move_to_end_of_line(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| {
                min(line.grapheme_count(), self.text_location.grapheme_index)
            });
    }

    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| {
                min(line.grapheme_count(), self.text_location.grapheme_index)
            });
    }

    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = min(self.text_location.line_index, self.buffer.height());
    }

    fn scroll_vertically(&mut self, to: usize) {
        let Size { height, .. } = self.size;

        let offset_changed = if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            true
        } else if to >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = to.saturating_sub(height).saturating_add(1);
            true
        } else {
            false
        };

        self.needs_redraw = self.needs_redraw || offset_changed;
    }

    fn scroll_horizontally(&mut self, to: usize) {
        let Size { width, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.col {
            self.scroll_offset.col = to;
            true
        } else if to >= self.scroll_offset.col.saturating_add(width) {
            self.scroll_offset.col = to.saturating_sub(width).saturating_add(1);
            true
        } else {
            false
        };

        self.needs_redraw = self.needs_redraw || offset_changed;
    }

    fn scroll_text_location_into_view(&mut self) {
        let Position { row, col } = self.text_location_to_position();

        self.scroll_vertically(row);
        self.scroll_horizontally(col);
    }

    fn render_buffer(&self) -> anyhow::Result<()> {
        let Size { height, width } = self.size;

        let top = self.scroll_offset.row;
        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {
                let truncated_line = {
                    let left = self.scroll_offset.col;
                    let right = self.scroll_offset.col.saturating_add(width);

                    line.get_visible_graphemes(left..right)
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
            text_location: Location::default(),
            scroll_offset: Position::default(),
        }
    }
}
