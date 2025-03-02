use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    terminal,
};

use anyhow::Context;

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Self {}
    }

    pub fn run(&self) -> anyhow::Result<()> {
        terminal::enable_raw_mode().context("couldn't enable raw mode")?;

        loop {
            if let Key(event) = event::read().context("couldn't read the keypress event")? {
                println!("{:?} \r", event);

                match event.code {
                    Char(c) => {
                        if c == 'q' {
                            break;
                        }
                    }
                    _ => (),
                }
            } else {
            }
        }

        terminal::disable_raw_mode().context("couldn't disable raw mode")?;

        Ok(())
    }
}
