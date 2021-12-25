use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders};

pub enum View {
    Splash,
    Home,
    Login,
}

pub enum Component {
    None,
}

pub struct Route {
    view: View,
    focus: Component,
}

impl Default for Route {
    fn default() -> Self {
        Route {
            view: View::Splash,
            focus: Component::None,
        }
    }
}

impl Route {
    pub fn draw<B>(&self, f: &mut Frame<B>)
    where
        B: Backend,
    {
        match self.view {
            _ => self.draw_splash_view(f),
        }
    }

    fn draw_splash_view<B>(&self, f: &mut Frame<B>)
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
        // third chunk purposefully empty
    }
}
