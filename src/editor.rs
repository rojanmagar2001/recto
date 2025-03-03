use crossterm::event::{
    Event::{self, Key},
    KeyCode::Char,
    KeyEvent, KeyModifiers,
};

mod terminal;
mod view;

use anyhow::Context;
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
            self.evaluate_event(&event);
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
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
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> anyhow::Result<()> {
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            View::render()?;
            Terminal::move_caret_to(Position::default())?;
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
