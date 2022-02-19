use std::cell::Cell;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Widget};

use super::layout::{Align, Axis};

pub trait Component<W>
where
    W: Widget,
{
    fn widget(&self) -> W;
    fn area(&self) -> Rect;
}

#[derive(Clone, Copy)]
pub struct Padding {
    top: u16,
    right: u16,
    left: u16,
    bottom: u16,
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
pub struct BoxStyle {
    height: u16,
    width: u16,
    min_height: u16,
    min_width: u16,
    padding: Padding,
}

#[derive(Clone)]
pub struct Button<'a> {
    widget: Block<'a>,
    focused: Cell<bool>,
    container: Cell<Rect>,
    pub style: Cell<BoxStyle>,
}

impl<'a> Default for Button<'a> {
    fn default() -> Self {
        let widget = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
            .title("Button");

        Button {
            widget,
            focused: Cell::new(false),
            container: Cell::new(Rect::default()),
            style: Cell::new(BoxStyle {
                height: 4,
                width: 30,
                min_height: 0,
                min_width: 0,
                padding: Padding::default(),
            }),
        }
    }
}

impl<'a> Component<Block<'a>> for Button<'a> {
    fn widget(&self) -> Block<'a> {
        self.widget.clone()
    }

    fn area(&self) -> Rect {
        let container = self.container.get();
        let style = self.style.get();

        let mut area = container.clone();

        // height
        if style.height >= container.height {
            if container.height < style.min_height {
                area.height = style.min_height
            } else {
                area.height = container.height
            }
        } else {
            area.height = style.height
        }

        // width
        if style.width >= container.width {
            if container.width < style.min_width {
                area.width = style.min_width;
            } else {
                area.width = container.width
            }
        } else {
            area.width = style.width
        }

        area
    }
}

impl<'a> Button<'a> {
    pub fn new(container: Rect) -> Self {
        let widget = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
            .title("Button");

        Button {
            widget,
            focused: Cell::new(false),
            container: Cell::new(container),
            style: Cell::new(BoxStyle {
                height: 4,
                width: 30,
                min_width: 0,
                min_height: 0,
                padding: Padding::default(),
            }),
        }
    }

    pub fn set_container(self, area: Rect) -> Self {
        self.container.set(area);
        self
    }

    pub fn set_height(self, pixels: u16) -> Self {
        let mut d = self.style.get();
        d.height = pixels;
        self.style.set(d);
        self
    }

    pub fn get_height(&self) -> u16 {
        self.style.get().height
    }

    pub fn set_width(self, pixels: u16) -> Self {
        let mut d = self.style.get();
        d.width = pixels;
        self.style.set(d);
        self
    }

    pub fn get_width(&self) -> u16 {
        self.style.get().width
    }

    pub fn focus(self, value: bool) -> Self {
        self.focused.set(value);

        self
    }
}
