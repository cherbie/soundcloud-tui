use std;

pub enum Route {
    Splash,
    Home,
    Login
}

pub struct App {
    pub route: Route,
}

impl Default for App {
    fn default() -> Self {
        App {
            route: Route::Splash
        }
    }
}