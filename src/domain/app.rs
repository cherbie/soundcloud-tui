use crate::event;
use crate::event::EventServer;

use super::context::*;
use super::router::draw;
use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;
use tui::backend::CrosstermBackend;
use tui::terminal::Terminal;

pub async fn render() -> Result<()> {
    let mut terminal = init_terminal()?;
    let context = Context::default();

    let mut event_server = event::EventServerCore::default();
    event_server.listen(event::crossterm::CrosstermEventSource);

    loop {
        draw(&context, &mut terminal)?;

        match event_server.next() {
            Some(event::Event::Input(crossterm::event::Event::Key(_))) => {
                break;
            }
            Some(event::Event::Tick) => {}
            _ => {}
        };
    }

    event_server.stop();
    exit_ui()?;

    Ok(())
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    terminal.hide_cursor()?;

    Ok(terminal)
}

fn exit_ui() -> Result<()> {
    execute!(io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
