use std::{
    env,
    panic::{set_hook, take_hook},
};

use crossterm::event::Event::{self};

mod editorcommand;
mod terminal;
mod view;

use anyhow::Context;
use editorcommand::{Direction, EditorCommand};
use terminal::Size;
use view::{location::Location, View};

use crate::editor::terminal::{Position, Terminal};

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Editor {
    pub fn new() -> anyhow::Result<Self> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            Terminal::terminate().unwrap();
            current_hook(panic_info);
        }));

        let mut view = View::default();

        let args = env::args().collect::<Vec<String>>();

        if let Some(file) = args.get(1) {
            view.load(file)?;
        }

        Ok(Self {
            should_quit: false,
            view,
        })
    }

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
            self.evaluate_event(event)?;
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: Event) -> anyhow::Result<()> {
        if let Ok(command) = EditorCommand::try_from(event) {
            match command {
                EditorCommand::Quit => {
                    self.should_quit = true;
                }
                EditorCommand::Resize(size) => {
                    self.view.resize(size);
                }
                EditorCommand::Move(direction) => {
                    self.move_point(direction)?;
                }
            }
        }

        Ok(())
    }

    fn move_point(&mut self, direction: Direction) -> anyhow::Result<()> {
        let Location { mut x, mut y } = self.view.location;
        let Size { width, height } = Terminal::size()?;

        match direction {
            Direction::Up => {
                y = y.saturating_sub(1);
            }
            Direction::Down => {
                y = y.saturating_add(1);
            }
            Direction::Left => {
                x = x.saturating_sub(1);
            }
            Direction::Right => {
                x = x.saturating_add(1);
            }
            Direction::PageUp => {
                y = 0;
            }
            Direction::PageDown => {
                y = height.saturating_sub(1);
            }
            Direction::End => {
                x = 0;
            }
            Direction::Home => {
                x = width.saturating_sub(1);
            }
        }

        self.view.location = Location { x, y };
        self.view.scroll_location_into_view();

        Ok(())
    }

    fn refresh_screen(&mut self) -> anyhow::Result<()> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_caret_to(self.view.get_postion())?;
        }

        Terminal::show_caret()?;
        Terminal::execute()?;

        Ok(())
    }
}
