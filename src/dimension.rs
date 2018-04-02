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
pub struct FloatRect {
    pub pos: Pos,
    pub size: Size,
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

pub const FLOAT_POS_ZERO : Pos = Pos {x : 0.0, y : 0.0};
pub const FLOAT_POS_CENTER : Pos = Pos {x : 0.5, y : 0.5};
pub const FLOAT_SIZE_ZERO : Size = Size {width : 0.0, height : 0.0};
pub const FLOAT_SIZE_HALF : Size = Size {width : 0.5, height : 0.5};
pub const FLOAT_SIZE_FULL : Size = Size {width : 1.0, height : 1.0};
pub const FLOAT_RECT_ZERO : FloatRect = FloatRect {pos : FLOAT_POS_ZERO, size : FLOAT_SIZE_ZERO};
pub const FLOAT_RECT_FULL : FloatRect = FloatRect {pos : FLOAT_POS_ZERO, size : FLOAT_SIZE_FULL};

pub const FIX_SIZE_ZERO : FixSize = FixSize {width : 0, height : 0};
pub const FIX_POS_ZERO : FixPos = FixPos {x : 0, y : 0};
pub const FIX_RECT_ZERO : FixRect = FixRect {pos : FIX_POS_ZERO, size : FIX_SIZE_ZERO};