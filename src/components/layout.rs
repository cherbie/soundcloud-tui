use tui::layout::Rect;

pub enum Axis {
    Mutual,
    Vertical,
    Horizontal,
}

pub trait Align {
    fn center(&mut self, area: Rect, axis: Axis) -> Self;
    fn left(&mut self, area: Rect) -> Self;
    fn right(&mut self, area: Rect) -> Self;
    fn top(&mut self, area: Rect) -> Self;
    fn bottom(&mut self, area: Rect) -> Self;
}
