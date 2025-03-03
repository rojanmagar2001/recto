use std::cmp::min;

use crossterm::event::{
    Event::{self, Key},
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};

mod terminal;
mod view;

use anyhow::Context;
use terminal::Size;
use view::View;

use crate::editor::terminal::{Position, Terminal};

#[derive(Debug, Clone, Copy, Default)]
pub struct Location {
    x: usize,
    y: usize,
}

pub struct Editor {
    should_quit: bool,
    view: View,
    location: Location,
}

impl Editor {
    pub fn run(&mut self) -> anyhow::Result<()> {
        Terminal::initialize()?;
        self.repl()?;
        Terminal::terminate()?;
        Ok(())
    }

    fn repl(&mut self) -> anyhow::Result<()> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }

            let event = crossterm::event::read().context("couldn't read the keypress event")?;
            self.evaluate_event(&event)?;
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> anyhow::Result<()> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: _,
            state: _,
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_point(&code)?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn move_point(&mut self, key_code: &KeyCode) -> anyhow::Result<()> {
        let Location { mut x, mut y } = self.location;
        let Size { width, height } = Terminal::size()?;

        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::End => {
                x = 0;
            }
            KeyCode::Home => {
                x = width.saturating_sub(1);
            }
            _ => {}
        }

        self.location = Location { x, y };

        Ok(())
    }

    fn refresh_screen(&self) -> anyhow::Result<()> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_caret_to(Position::new(self.location.x, self.location.y))?;
        }

        Terminal::show_caret()?;
        Terminal::execute()?;

        Ok(())
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            should_quit: false,
            view: View::default(),
            location: Location::default(),
        }
    }
}
