use super::App;
use crate::event;
use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders};
use tui::Terminal;

pub async fn render(app: &mut App) -> Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    loop {
        terminal.draw(|f| {
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
        })?;

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

impl App {
    pub fn exit_ui(&self) -> Result<()> {
        execute!(io::stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}

impl Drop for App {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        self.exit_ui();
    }
}
