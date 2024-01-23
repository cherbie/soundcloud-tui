use crate::components::style::{Alignment, BorderStyle, BoxStyle, FlexBox};
use crate::components::Component;
use std::cell::Cell;
use tui::layout::Rect;
use tui::widgets::{Block, BorderType, Borders};

#[derive(Clone)]
pub struct Button<'a> {
    widget: Block<'a>,
    pub border_title: Option<&'a str>,
    pub inner_text: Option<&'a str>,
    pub focused: Cell<bool>,
    pub style: Cell<BoxStyle>,
}

impl<'a> Default for Button<'a> {
    fn default() -> Self {
        Button {
            widget: Block::default(),
            border_title: None,
            inner_text: None,
            focused: Cell::new(false),
            style: Cell::new(
                *BoxStyle::default()
                    .height(4)
                    .width(30)
                    .border_style(BorderStyle {
                        fg: None,
                        bg: None,
                        decorations: None,
                        borders: Borders::ALL,
                        border_type: BorderType::Rounded,
                    })
                    .flex(FlexBox {
                        justify_content: Alignment::Center,
                        align_items: Alignment::Center,
                    }),
            ),
        }
    }
}

impl<'a> Component<Block<'a>> for Button<'a> {
    fn widget(&self) -> Block<'a> {
        let style = self.style.get();
        let widget = self
            .widget
            .clone()
            .borders(style.border_style.borders)
            .border_style(style.border_style.into())
            .border_type(style.border_style.border_type)
            .style(style.text_style.into())
            .title("Button");
        if self.border_title.is_some() {
            return widget.title(self.border_title.unwrap());
        }
        widget
    }

    fn area(&self, container: Rect) -> Rect {
        let style = self.style.get();
        let mut area = container;

        area.height -= style.padding.top + style.padding.bottom;
        area.width -= style.padding.left + style.padding.right;

        // height
        if style.height >= area.height {
            if area.height < style.min_height {
                area.height = style.min_height
            }
        } else {
            area.height = style.height
        }

        // width
        if style.width >= area.width {
            if area.width < style.min_width {
                area.width = style.min_width;
            }
        } else {
            area.width = style.width
        }

        match style.flex.justify_content {
            Alignment::Center => area.x = (container.x + container.width / 2) - area.width / 2,
            Alignment::End => area.x = area.x + container.width - area.width,
            _ => (),
        }

        match style.flex.align_items {
            Alignment::Center => area.y = (container.y + container.height / 2) - area.height / 2,
            Alignment::End => area.y = container.y + container.height - area.height,
            _ => (),
        }

        area
    }
}
