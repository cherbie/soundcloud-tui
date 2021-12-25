use tui::layout::Rect;

pub trait Centered {
    fn center(area: &Rect) -> Rect;
}
