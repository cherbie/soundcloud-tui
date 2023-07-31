use std::convert::{From, Into};
use std::option::Option;
use tui::style::{Color, Modifier, Style};

#[derive(Clone, Copy)]
pub struct TextStyle {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub decorations: Option<Modifier>,
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle {
            decorations: None,
            fg: None,
            bg: None,
        }
    }
}

impl From<Style> for TextStyle {
    fn from(style: Style) -> Self {
        let mut text_style = TextStyle::default();
        text_style.fg = style.fg;
        text_style.bg = style.bg;
        text_style.decorations = Some(style.add_modifier);

        text_style
    }
}

impl Into<Style> for TextStyle {
    fn into(self) -> Style {
        let style = Style::default();
        if self.fg.is_some() {
            style.fg(self.fg.unwrap());
        }
        if self.bg.is_some() {
            style.bg(self.bg.unwrap());
        }
        if self.decorations.is_some() {
            style.add_modifier(self.decorations.unwrap());
        }
        style
    }
}
