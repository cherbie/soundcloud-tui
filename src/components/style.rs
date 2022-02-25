use std::convert::{From, Into};
use std::option::Option;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{BorderType, Borders};

#[derive(Clone, Copy)]
pub struct Padding {
    pub top: u16,
    pub right: u16,
    pub left: u16,
    pub bottom: u16,
}

impl Default for Padding {
    fn default() -> Self {
        Padding {
            top: 0,
            right: 0,
            left: 0,
            bottom: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Alignment {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy)]
pub struct FlexBox {
    pub justify_content: Alignment,
    pub align_items: Alignment,
}

impl Default for FlexBox {
    fn default() -> Self {
        FlexBox {
            justify_content: Alignment::Start,
            align_items: Alignment::Start,
        }
    }
}

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

#[derive(Clone, Copy, Default)]
pub struct BoxStyle {
    pub height: u16,
    pub width: u16,
    pub min_height: u16,
    pub min_width: u16,
    pub padding: Padding,
    pub flex: FlexBox,
    pub border_style: BorderStyle,
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
    pub text_style: TextStyle,
}

impl From<Style> for BoxStyle {
    fn from(style: Style) -> Self {
        let mut box_style = BoxStyle::default();
        box_style.text_style.decorations = Some(style.add_modifier);
        box_style.fg_color = style.fg;
        box_style.bg_color = style.bg;

        box_style
    }
}

impl BoxStyle {
    pub fn height(&mut self, height: u16) -> &mut Self {
        self.height = height;
        self
    }

    pub fn width(&mut self, width: u16) -> &mut Self {
        self.width = width;
        self
    }

    pub fn min_height(&mut self, height: u16) -> &mut Self {
        self.min_height = height;
        self
    }

    pub fn min_width(&mut self, width: u16) -> &mut Self {
        self.min_width = width;
        self
    }

    pub fn padding(&mut self, padding: [u16; 4]) -> &mut Self {
        self.padding = Padding {
            top: padding[0],
            right: padding[1],
            bottom: padding[2],
            left: padding[3],
        };
        self
    }

    pub fn flex(&mut self, flex: FlexBox) -> &mut Self {
        self.flex = flex;
        self
    }

    pub fn border_style(&mut self, style: BorderStyle) -> &mut Self {
        self.border_style = style;
        self
    }
}
