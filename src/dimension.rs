#[derive(Debug, Copy, Clone)]
pub struct FloatSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct FloatPos {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct FloatRect {
    pub pos: FloatPos,
    pub size: FloatSize,
}

#[derive(Debug, Copy, Clone)]
pub struct AnchorPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct  Gravity {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Scale {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub pos: Pos,
    pub size: Size,
}



pub const FLOAT_POS_ZERO : FloatPos = FloatPos {x : 0.0, y : 0.0};
pub const FLOAT_POS_CENTER : FloatPos = FloatPos {x : 0.5, y : 0.5};
pub const FLOAT_SIZE_ZERO : FloatSize = FloatSize {width : 0.0, height : 0.0};
pub const FLOAT_SIZE_HALF : FloatSize = FloatSize {width : 0.5, height : 0.5};
pub const FLOAT_SIZE_FULL : FloatSize = FloatSize {width : 1.0, height : 1.0};
pub const FLOAT_RECT_ZERO : FloatRect = FloatRect {pos : FLOAT_POS_ZERO, size : FLOAT_SIZE_ZERO};
pub const FLOAT_RECT_FULL : FloatRect = FloatRect {pos : FLOAT_POS_ZERO, size : FLOAT_SIZE_FULL};

pub const SIZE_ZERO: Size = Size {width : 0, height : 0};
pub const POS_ZERO: Pos = Pos {x : 0, y : 0};
pub const RECT_ZERO: Rect = Rect {pos : POS_ZERO, size : SIZE_ZERO };

pub const ANCHOR_POINT_CENTER:AnchorPoint = AnchorPoint {x : 0.5, y : 0.5};
pub const ANCHOR_POINT_TOP_LEFT:AnchorPoint = AnchorPoint {x : 0.0, y : 0.0};
pub const ANCHOR_POINT_MIDDLE_LEFT:AnchorPoint = AnchorPoint {x : 0.0, y : 0.5};
pub const ANCHOR_POINT_BOTTOM_LEFT:AnchorPoint = AnchorPoint {x : 0.0, y : 1.0};
pub const ANCHOR_POINT_TOP_RIGHT:AnchorPoint = AnchorPoint {x : 1.0, y : 0.0};
pub const ANCHOR_POINT_MIDDLE_RIGHT:AnchorPoint = AnchorPoint {x : 0.1, y : 0.5};
pub const ANCHOR_POINT_BOTTOM_RIGHT:AnchorPoint = AnchorPoint {x : 1.0, y : 1.0};
pub const ANCHOR_POINT_TOP_CENTER:AnchorPoint = AnchorPoint {x : 0.5, y : 0.0};
pub const ANCHOR_POINT_BOTTOM_CENTER:AnchorPoint = AnchorPoint {x : 0.5, y : 1.0};

pub const GRAVITY_CENTER:Gravity = Gravity {x : 0.5, y : 0.5};
pub const GRAVITY_TOP_LEFT:Gravity = Gravity {x : 0.0, y : 0.0};
pub const GRAVITY_MIDDLE_LEFT:Gravity = Gravity {x : 0.0, y : 0.5};
pub const GRAVITY_BOTTOM_LEFT:Gravity = Gravity {x : 0.0, y : 1.0};
pub const GRAVITY_TOP_RIGHT:Gravity = Gravity {x : 1.0, y : 0.0};
pub const GRAVITY_MIDDLE_RIGHT:Gravity = Gravity {x : 0.1, y : 0.5};
pub const GRAVITY_BOTTOM_RIGHT:Gravity = Gravity {x : 1.0, y : 1.0};
pub const GRAVITY_TOP_CENTER:Gravity = Gravity {x : 0.5, y : 0.0};
pub const GRAVITY_BOTTOM_CENTER:Gravity = Gravity {x : 0.5, y : 1.0};

pub const SCALE_SINGLE: Scale = Scale {x : 1.0, y : 1.0};
