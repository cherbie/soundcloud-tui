use super::router::Route;

pub use api::AppContext;
pub use lib::*;

mod api {
    use super::*;

    pub trait AppContext {
        fn get_route(&self) -> Route;
    }
}

mod lib {
    use super::*;

    pub struct Context {
        route: Route,
    }

    /// AppContext ///
    impl AppContext for Context {
        fn get_route(&self) -> Route {
            self.route
        }
    }

    /// Default ///
    impl Default for Context {
        fn default() -> Self {
            Context {
                route: Route::Splash,
            }
        }
    }
}
