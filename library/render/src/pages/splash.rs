use tui::backend::Backend;
use tui::terminal::Frame;

use crate::components::style::layout::{Dom, DomNode};
use crate::components::widgets::Button;
use crate::context::AppContext;

pub fn draw_splash_view<B>(_context: &dyn AppContext, f: &mut Frame<B>)
where
    B: Backend,
{
    let dom = Dom::default();
    dom.root.borrow_mut().container.set(f.size());
    DomNode::add_child(dom.root.clone(), Box::<Button<'_>>::default(), None);
    Dom::render(f, &dom.root);
}
