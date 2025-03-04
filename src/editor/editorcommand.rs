use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use super::terminal::Size;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    End,
    Home,
}

pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Quit,
}

impl TryFrom<Event> for EditorCommand {
    type Error = anyhow::Error;

    fn try_from(event: Event) -> anyhow::Result<Self> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                // Quit Command
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                // Move
                (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                _ => Err(anyhow::format_err!("Key Code not supported: {code:?}")),
            },
            Event::Resize(width_u16, height_u16) => Ok(Self::Resize(Size {
                width: width_u16 as usize,
                height: height_u16 as usize,
            })),
            _ => Err(anyhow::format_err!("Event not supported: {event:?}")),
        }
    }
}
