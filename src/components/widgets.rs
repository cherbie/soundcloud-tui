use super::Component;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::canvas::Label;
use tui::widgets::{Block, BorderType, Borders};

#[derive(Clone)]
pub struct Button<'a> {
    widget: Block<'a>,
    focused: bool,
    pub area: Rect,
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
            focused: false,
            area: Rect::default(),
        }
    }
}

impl<'a> Component<Block<'a>> for Button<'a> {
    fn get_widget(&self) -> Block<'a> {
        self.widget.clone()
    }
}

impl<'a> Button<'a> {
    pub fn set_area(mut self, area: Rect) -> Button<'a> {
        self.area = area;
        self
    }

    pub fn focus(mut self, is_focused: bool) -> Button<'a> {
        self.focused = is_focused;
        self
    }
}
