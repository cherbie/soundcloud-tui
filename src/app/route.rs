use super::layout::Dom;

pub enum View {
    Splash,
    Home,
    Login,
}

pub struct Route {
    pub view: View,
    pub dom: Dom,
}

impl Default for Route {
    fn default() -> Self {
        Route {
            view: View::Splash,
            dom: Dom::default(),
        }
    }
}
