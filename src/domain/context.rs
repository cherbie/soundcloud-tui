use anyhow::Result;
use std::cell::RefCell;
use std::io;
use tui::backend::{Backend, CrosstermBackend};
use tui::terminal::Terminal;

use super::router::Router;

pub struct Context<B>
where
    B: Backend,
{
    terminal: RefCell<Terminal<B>>,
    router: Router,
}

impl Context<CrosstermBackend<io::Stdout>> {
    pub fn new() -> Result<Self> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        Ok(Context {
            terminal: RefCell::new(terminal),
            router: Router::default(),
        })
    }
}
