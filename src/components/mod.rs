pub mod layout;
mod widgets;

pub trait Component<W>
where
    W: Widget,
{
    fn get_widget(&self) -> W;
}

use tui::widgets::Widget;

pub use self::widgets::Button;
