
use screen_writer::{ScreenInfo};
use color::{Color};
use dimension::{FixRect, FixSize, FIX_RECT_ZERO, FIX_POS_ZERO};
use sprite::{Sprite, render_to_canvas};

pub struct RectSprite {
    pub color: Color,
    rect : FixRect,
    raw_pixels : Vec<u32>,
}

impl RectSprite {
    pub fn new(color: Color) -> RectSprite {
        RectSprite {
            color : color,
            rect : FIX_RECT_ZERO,
            raw_pixels : Vec::new(),
        }
    }
}

impl<'a> Sprite<'a> for RectSprite {

    fn draw(&mut self, fixed_rect:&FixRect, _screen_info:&ScreenInfo) {
        self.rect = *fixed_rect;
        let size = (self.rect.size.width * self.rect.size.height) as usize;
        self.raw_pixels = vec![self.color.rgba_as_u32; size];
    }

    fn render(&mut self, fixed_rect:&FixRect, screen_info:&ScreenInfo, canvas_ptr:*mut u32) {
        let src_slice_ptr_u32 = self.raw_pixels.as_ptr() as *mut u32;
        let inner_frame = FixRect { pos: FIX_POS_ZERO, size: FixSize { width: self.rect.size.width, height : self.rect.size.height}};
        render_to_canvas(src_slice_ptr_u32, fixed_rect, &inner_frame, screen_info, canvas_ptr);
    }
}
