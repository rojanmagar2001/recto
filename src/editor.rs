use crossterm::{
    event::{self, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers},
    terminal,
};

use anyhow::Context;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        self.repl()?;
        Ok(())
    }

    pub fn repl(&mut self) -> anyhow::Result<()> {
        terminal::enable_raw_mode().context("couldn't enable raw mode")?;

        loop {
            if let Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = event::read().context("couldn't read the keypress event")?
            {
                println!(
                    "Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r"
                );

                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    _ => (),
                }
            }

            if self.should_quit {
                break;
            }
        }

        terminal::disable_raw_mode().context("couldn't disable raw mode")?;

        Ok(())
    }
}
