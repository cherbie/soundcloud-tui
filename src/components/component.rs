use tui::layout::Rect;
use tui::widgets::Widget;

pub trait Component<W>
where
    W: Widget,
{
    fn widget(&self) -> W;
    fn area(&self, container: Rect) -> Rect;
}
