
use screen_writer::{ScreenInfo};
use color;
use dimension::{Rect, Size, RECT_ZERO, POS_ZERO};
use sprite::{Sprite, render_to_canvas};

pub struct RectSprite {
    pub color: color::Color,
    rect : Rect,
    raw_pixels : Vec<u32>,
}

impl RectSprite {
    pub fn new() -> RectSprite {
        RectSprite {
            color : color::BLACK,
            rect : RECT_ZERO,
            raw_pixels : Vec::new(),
        }
    }
}

impl<'a> Sprite<'a> for RectSprite {

    fn draw(&mut self, fixed_rect:&Rect, _screen_info:&ScreenInfo) {
        self.rect = *fixed_rect;
        let size = (self.rect.size.width * self.rect.size.height) as usize;
        let color::Color(c) = self.color;
        self.raw_pixels = vec![c; size];
    }

    fn render(&mut self, parent_node_frame:&Rect, node_frame:&Rect, clip_to_bounds:bool, screen_info:&ScreenInfo, canvas_ptr:*mut u32) {
        let src_slice_ptr_u32 = self.raw_pixels.as_ptr() as *mut u32;
        let inner_frame = Rect { pos: POS_ZERO, size: Size { width: self.rect.size.width, height : self.rect.size.height}};

        render_to_canvas(src_slice_ptr_u32, parent_node_frame,node_frame, &inner_frame,clip_to_bounds, screen_info, canvas_ptr);
    }
}
