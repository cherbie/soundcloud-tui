// TODO: implement
use std::convert::From;
use std::option::Option;
use tui::style::{Color, Style};

use super::border::BorderStyle;
use super::text::TextStyle;

#[derive(Clone, Copy, Default)]
pub struct Padding {
    pub top: u16,
    pub right: u16,
    pub left: u16,
    pub bottom: u16,
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
