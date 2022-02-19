use super::App;
use crate::event;
use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;
use tui::backend::Backend;

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
        self.terminal.hide_cursor()?;

        Ok(())
    }

    pub fn exit_ui(&self) -> Result<()> {
        execute!(io::stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        self.terminal.draw(|f| {
            self.route.draw(f);
        })?;

        Ok(())
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
