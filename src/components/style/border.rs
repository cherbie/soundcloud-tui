use std::convert::{From, Into};
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
        BorderStyle {
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
        let mut border_style = BorderStyle::default();
        border_style.fg = style.fg;
        border_style.bg = style.bg;
        border_style.decorations = Some(style.add_modifier);

        border_style
    }
}

impl Into<Style> for BorderStyle {
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
