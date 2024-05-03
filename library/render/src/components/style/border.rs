use std::convert::From;
use std::option::Option;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{BorderType, Borders};

#[derive(Clone, Copy)]
pub struct BorderStyle {
    pub borders: Borders,
    pub border_type: BorderType,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub decorations: Option<Modifier>,
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            borders: Borders::NONE,
            border_type: BorderType::Plain,
            fg: None,
            bg: None,
            decorations: None,
        }
    }
}

impl From<Style> for BorderStyle {
    fn from(style: Style) -> Self {
        Self {
            fg: style.fg,
            bg: style.bg,
            ..Default::default()
        }
    }
}

impl From<BorderStyle> for Style {
    fn from(border_style: BorderStyle) -> Self {
        Self {
            fg: border_style.fg,
            bg: border_style.bg,
            add_modifier: border_style.decorations.unwrap_or(Modifier::empty()),
            ..Default::default()
        }
    }
}
