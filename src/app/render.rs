use super::layout::{Dom, DomNode};
use super::route::View;
use super::App;
use crate::components::widgets::Button;
use crate::event;

use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders};

pub async fn render<B>(app: &mut App<B>) -> Result<()>
where
    B: Backend,
{
    app.start_ui()?;

    loop {
        app.draw()?;

        match app.events.lock().unwrap().next()? {
            event::Event::Input(key) => {
                if key == event::Key::Ctrl('c') {
                    return Ok(());
                }
            }
            event::Event::Tick => {}
        };
    }
}

impl<B> App<B>
where
    B: Backend,
{
    pub fn start_ui(&mut self) -> Result<()> {
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        enable_raw_mode()?;
        self.terminal.get_mut().hide_cursor()?;

        Ok(())
    }

    pub fn exit_ui(&self) -> Result<()> {
        execute!(io::stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }

    pub fn draw(&self) -> Result<()> {
        self.terminal.borrow_mut().draw(|f| match self.route.view {
            View::Splash => self.draw_splash_view(f),
            View::Home => self.draw_home_view(f),
            View::Login => self.draw_login_view(f),
        })?;

        Ok(())
    }

    fn draw_splash_view(&self, f: &mut Frame<B>)
    where
        B: Backend,
    {
        self.route.dom.root.borrow_mut().container.set(f.size());
        DomNode::add_child(
            self.route.dom.root.clone(),
            Box::new(Button::default()),
            None,
        );
        Dom::render(f, &self.route.dom.root);
    }

    fn draw_home_view(&self, f: &mut Frame<B>)
    where
        B: Backend,
    {
        ()
    }

    fn draw_login_view(&self, f: &mut Frame<B>)
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

impl<B> Drop for App<B>
where
    B: Backend,
{
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        self.exit_ui();
    }
}
