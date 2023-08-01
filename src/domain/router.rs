use crate::components::style::layout::Dom;
use crate::components::views::View;

pub struct Router {
    pub view: View,
    pub dom: Dom,
}

impl Default for Router {
    fn default() -> Self {
        Router {
            view: View::Splash,
            dom: Dom::default(),
        }
    }
}
