use std::cell::Cell;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders};

use super::style::{BoxStyle, FlexBox, Padding};
use super::Component;

#[derive(Clone)]
pub struct Button<'a> {
    widget: Block<'a>,
    pub focused: Cell<bool>,
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
                flex: FlexBox::default(),
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
                flex: FlexBox::default(),
            }),
        }
    }

    pub fn set_container(self, area: Rect) -> Self {
        self.container.set(area);
        self
    }
}
