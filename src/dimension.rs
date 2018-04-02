#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct AnchorPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct FixPos {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct FixSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct FixRect {
    pub pos: FixPos,
    pub size: FixSize,
}

pub const FIX_SIZE_ZERO : FixSize = FixSize {width : 0, height : 0};
pub const FIX_POS_ZERO : FixPos = FixPos {x : 0, y : 0};
pub const FIX_RECT_ZERO : FixRect = FixRect {pos : FIX_POS_ZERO, size : FIX_SIZE_ZERO};