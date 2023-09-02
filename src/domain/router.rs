use super::{
    context::AppContext,
    pages::{draw_login_view, draw_splash_view},
};
use std::io::Result;
use tui::{backend::Backend, terminal::Terminal};

#[derive(Debug, Copy, Clone)]
pub enum Route {
    Splash,
    Home,
    Login,
}

pub fn draw<B>(context: &dyn AppContext, terminal: &mut Terminal<B>) -> Result<()>
where
    B: Backend,
{
    terminal.draw(|f| match context.get_route() {
        Route::Splash => draw_splash_view(context, f),
        Route::Home => (),
        Route::Login => draw_login_view(context, f),
    })?;

    Ok(())
}
