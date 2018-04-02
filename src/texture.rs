extern crate image;

use std::fs;
use std::io::{BufReader};
use image::{GenericImage, DynamicImage, imageops};

use dimension::*;
use screen_writer::{ScreenInfo};
use sprite::{Sprite, render_to_canvas};

pub struct TextureSprite {
    texture : DynamicImage,
    pub gravity : Gravity,
    size : FixSize,
    raw_pixels : Vec<u8>,
    frame : FixRect,
}

impl TextureSprite {

    pub fn new_for_texture(filename: &str) -> TextureSprite {

        let file: fs::File = fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let load_result = image::load(reader, image::PNG).unwrap();

        TextureSprite {
            raw_pixels : load_result.raw_pixels(),
            gravity : GRAVITY_CENTER,
            size : FixSize {
                width : load_result.width(),
                height : load_result.height(),
            },
            texture : load_result,
            frame : FIX_RECT_ZERO,
        }
    }
}

impl<'a> Sprite<'a> for TextureSprite {

    fn draw(&mut self, outer_rect:&FixRect, _screen_info:&ScreenInfo) {
        let image = &self.texture;

        let mut width = outer_rect.size.width;
        let mut height = outer_rect.size.height;

        if image.width() != outer_rect.size.width && image.height() != outer_rect.size.height {
            width = outer_rect.size.width.min(width);
            height = outer_rect.size.height.min(height);
            let new_image = image.resize(width, height, imageops::Gaussian);
            self.raw_pixels = new_image.raw_pixels();
            self.size = FixSize {
                width : new_image.width(),
                height : new_image.height(),
            };
            width = new_image.width();
            height = new_image.width();
        }

        let mut frame = FixRect {pos : FIX_POS_ZERO, size : FixSize {width : width, height : height}};

        frame.pos.x = ((outer_rect.size.width as f32 * self.gravity.x) - (frame.size.width as f32 * self.gravity.x)) as u32;
        frame.pos.y = ((outer_rect.size.height as f32 * self.gravity.y) - (frame.size.height as f32 * self.gravity.y)) as u32;

        self.frame = frame;

    }

    fn render(&mut self, fixed_rect:&FixRect, screen_info:&ScreenInfo, canvas_ptr:*mut u32) {
        let src_slice_ptr_u32 = self.raw_pixels.as_slice().as_ptr() as *mut u32;
        render_to_canvas(src_slice_ptr_u32, fixed_rect, &self.frame, screen_info, canvas_ptr);
    }
}
