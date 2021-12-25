use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders};

use crate::components::{Button, Component};

pub enum View {
    Splash,
    Home,
    Login,
}

pub enum Element {
    None,
}

pub struct Route {
    view: View,
    focus: Element,
}

impl Default for Route {
    fn default() -> Self {
        Route {
            view: View::Splash,
            focus: Element::None,
        }
    }
}

impl Route {
    pub fn draw<B>(&self, f: &mut Frame<B>)
    where
        B: Backend,
    {
        match self.view {
            View::Splash => self.draw_splash_view(f),
            View::Home => self.draw_home_view(f),
            View::Login => self.draw_login_view(f),
        }
    }

    fn draw_splash_view<B>(&self, f: &mut Frame<B>)
    where
        B: Backend,
    {
        let canvas = f.size();
        let height: u16 = std::cmp::min(4, canvas.height);
        let width: u16 = std::cmp::min(30, canvas.width);
        let button = Button::default().set_area(Rect {
            x: (canvas.width - width) / 2,
            y: (canvas.height - height) / 2,
            width,
            height,
        });
        f.render_widget(button.get_widget(), button.area);
    }

    fn draw_home_view<B>(&self, f: &mut Frame<B>)
    where
        B: Backend,
    {
    }

    fn draw_login_view<B>(&self, f: &mut Frame<B>)
    where
        B: Backend,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(f.size());
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default().title("Block 2").borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    }
}
