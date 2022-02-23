#[derive(Clone, Copy)]
pub struct Padding {
    pub top: u16,
    pub right: u16,
    pub left: u16,
    pub bottom: u16,
}

impl Default for Padding {
    fn default() -> Self {
        Padding {
            top: 0,
            right: 0,
            left: 0,
            bottom: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Alignment {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy)]
pub struct FlexBox {
    pub justify_content: Alignment,
    pub align_items: Alignment,
}

impl Default for FlexBox {
    fn default() -> Self {
        FlexBox {
            justify_content: Alignment::Start,
            align_items: Alignment::Start,
        }
    }
}

#[derive(Clone, Copy)]
pub struct BoxStyle {
    pub height: u16,
    pub width: u16,
    pub min_height: u16,
    pub min_width: u16,
    pub padding: Padding,
    pub flex: FlexBox,
}
