use std::convert::From;
use std::option::Option;
use tui::style::{Color, Modifier, Style};

#[derive(Clone, Copy, Default)]
pub struct TextStyle {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub decorations: Option<Modifier>,
}

impl From<Style> for TextStyle {
    fn from(style: Style) -> Self {
        Self {
            fg: style.fg,
            bg: style.bg,
            ..Default::default()
        }
    }
}

impl From<TextStyle> for Style {
    fn from(val: TextStyle) -> Self {
        let style = Style::default();
        if val.fg.is_some() {
            style.fg(val.fg.unwrap());
        }
        if val.bg.is_some() {
            style.bg(val.bg.unwrap());
        }
        if val.decorations.is_some() {
            style.add_modifier(val.decorations.unwrap());
        }
        style
    }
}
