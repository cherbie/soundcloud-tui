mod render;

use crate::event;
use crate::views::route::Route;
use anyhow::Result;
use std::cell::RefCell;
use std::io;
use std::sync::Mutex;
use tui::backend::{Backend, CrosstermBackend};
use tui::terminal::Terminal;

pub use self::render::render;

pub struct App<B>
where
    B: Backend,
{
    terminal: RefCell<Terminal<B>>,
    route: Route,
    pub events: Mutex<event::Events>,
}

impl App<CrosstermBackend<io::Stdout>> {
    pub fn new() -> Result<Self> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        Ok(App {
            terminal: RefCell::new(terminal),
            route: Route::default(),
            events: Mutex::new(event::Events::default()),
        })
    }
}
