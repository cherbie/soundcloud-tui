use crate::domain::context::AppContext;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    terminal::Frame,
    widgets::{Block, Borders},
};

pub fn draw_login_view<B>(_context: &dyn AppContext, f: &mut Frame<B>)
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
