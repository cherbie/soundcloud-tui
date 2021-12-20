mod render;
mod route;

use self::route::Route;
use crate::event;
use std::sync::Mutex;

pub use self::render::render;

pub struct App {
    pub route: Route,
    pub events: Mutex<event::Events>,
}

impl Default for App {
    fn default() -> Self {
        App {
            route: Route::Splash,
            events: Mutex::new(event::Events::default()),
        }
    }
}
