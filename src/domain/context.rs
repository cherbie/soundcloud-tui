use anyhow::Result;
use std::cell::RefCell;
use std::io;
use std::sync::Mutex;
use tui::backend::{Backend, CrosstermBackend};
use tui::terminal::Terminal;

use super::router::Router;
use crate::event;

pub struct Context<B>
where
    B: Backend,
{
    terminal: RefCell<Terminal<B>>,
    router: Router,
    pub events: Mutex<event::Events>,
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
            events: Mutex::new(event::Events::default()),
        })
    }
}
