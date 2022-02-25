use std::cell::Cell;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders};

use super::style::{Alignment, BoxStyle, FlexBox, Padding};
use super::Component;

#[derive(Clone)]
pub struct Button<'a> {
    widget: Block<'a>,
    pub focused: Cell<bool>,
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
            style: Cell::new(BoxStyle {
                height: 4,
                width: 30,
                min_height: 0,
                min_width: 0,
                padding: Padding::default(),
                flex: FlexBox {
                    justify_content: Alignment::Center,
                    align_items: Alignment::Center,
                },
            }),
        }
    }
}

impl<'a> Component<Block<'a>> for Button<'a> {
    fn widget(&self) -> Block<'a> {
        self.widget.clone()
    }

    fn area(&self, container: Rect) -> Rect {
        let style = self.style.get();
        let mut area = container;

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

// impl<'a> Button<'a> {
//     pub fn new() -> Self {
//         let widget = Block::default()
//             .borders(Borders::ALL)
//             .border_style(Style::default().fg(Color::White))
//             .border_type(BorderType::Rounded)
//             .title("Button");

//         Button {
//             widget,
//             focused: Cell::new(false),
//             style: Cell::new(BoxStyle {
//                 height: 4,
//                 width: 30,
//                 min_width: 0,
//                 min_height: 0,
//                 padding: Padding::default(),
//                 flex: FlexBox::default(),
//             }),
//         }
//     }
// }
