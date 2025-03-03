use std::{
    cmp::min,
    env,
    panic::{set_hook, take_hook},
};

use crossterm::event::{
    Event::{self},
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

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    location: Location,
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
            location: Location::default(),
            view,
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        Terminal::initialize()?;
        self.repl()?;
        Terminal::terminate()?;
        Ok(())
    }

    // fn handle_args(&mut self) -> anyhow::Result<()> {
    //     let args = env::args().collect::<Vec<String>>();
    //
    //     if let Some(file) = args.get(1) {
    //         self.view.load(file)?;
    //     }
    //
    //     Ok(())
    // }

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
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                (
                    KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::PageUp
                    | KeyCode::PageDown
                    | KeyCode::End
                    | KeyCode::Home,
                    _,
                ) => {
                    self.move_point(code)?;
                }
                _ => {}
            },
            Event::Resize(width_u16, height_u16) => {
                self.view.resize(Size {
                    width: width_u16 as usize,
                    height: height_u16 as usize,
                });
            }
            _ => {}
        }

        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> anyhow::Result<()> {
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

    fn refresh_screen(&mut self) -> anyhow::Result<()> {
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
