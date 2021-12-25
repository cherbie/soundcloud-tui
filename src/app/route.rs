use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::StatefulWidget;

pub enum Page {
    Splash,
    Home,
    Login,
}

pub enum Component {
    None,
}

pub struct Route {
    page: Page,
    focus: Component,
}

impl Default for Route {
    fn default() -> Self {
        Route {
            page: Page::Splash,
            focus: Component::None,
        }
    }
}

struct RouteWidget {
    route: Route,
}

impl RouteWidget {
    pub fn new(route: Route) -> Self {
        RouteWidget { route: route }
    }
}
