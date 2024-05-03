use crossterm::event;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub enum Key {
    Unknown,
    Esc,
    Enter,
    Ctrl(char),
    Alt(char),
    Char(char),
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Key::Esc => write!(f, "<Esc>"),

            Key::Enter => write!(f, "<Enter>"),

            Key::Ctrl(c) => write!(f, "<Ctrl+{}>", c),

            Key::Alt(c) => write!(f, "<Alt+{}>", c),

            Key::Char(c) => write!(f, "<Char: {}>", c),

            _ => write!(f, ":?"),
        }
    }
}

impl From<event::KeyEvent> for Key {
    fn from(key_event: event::KeyEvent) -> Self {
        match key_event {
            event::KeyEvent {
                code: event::KeyCode::Esc,
                ..
            } => Key::Esc,

            event::KeyEvent {
                code: event::KeyCode::Enter,
                ..
            } => Key::Enter,

            event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::ALT,
                ..
            } => Key::Alt('c'),

            event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => Key::Ctrl('c'),

            event::KeyEvent {
                code: event::KeyCode::Char('c'),
                ..
            } => Key::Char('c'),

            _ => Key::Unknown,
        }
    }
}
