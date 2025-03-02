use crossterm::event::{
    Event::{self, Key},
    KeyCode::Char,
    KeyEvent, KeyModifiers,
};

use anyhow::Context;

use crate::terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        Terminal::initialize()?;
        self.repl()?;
        Terminal::terminate()?;
        Ok(())
    }

    pub fn draw_rows() -> anyhow::Result<()> {
        let height = Terminal::size()?.1;

        for current_row in 0..height {
            Terminal::print("~")?;

            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }

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
                _ => (),
            }
        }

        Ok(())
    }

    fn refresh_screen(&self) -> anyhow::Result<()> {
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }

        Terminal::show_cursor()?;
        Terminal::execute()?;

        Ok(())
    }
}
