mod render;
mod route;

use self::route::Route;
use crate::event;
use anyhow::Result;
use std::io;
use std::sync::Mutex;
use tui::backend::{Backend, CrosstermBackend};
use tui::terminal::Terminal;

pub use self::render::render;

pub struct App<B>
where
    B: Backend,
{
    terminal: Terminal<B>,
    route: Route,
    pub events: Mutex<event::Events>,
}

impl App<CrosstermBackend<io::Stdout>> {
    pub fn new() -> Result<Self> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);

        Ok(App {
            terminal: Terminal::new(backend)?,
            route: Route::default(),
            events: Mutex::new(event::Events::default()),
        })
    }
}
