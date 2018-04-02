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
    raw_pixels : Vec<u8>,
    frame : Rect,
}

impl TextureSprite {

    pub fn new_for_texture(filename: &str) -> TextureSprite {

        let file: fs::File = fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let load_result = image::load(reader, image::PNG).unwrap();

        TextureSprite {
            raw_pixels : load_result.raw_pixels(),
            gravity : GRAVITY_CENTER,
            texture : load_result,
            frame : RECT_ZERO,
        }
    }
}

impl<'a> Sprite<'a> for TextureSprite {

    fn draw(&mut self, outer_rect:&Rect, _screen_info:&ScreenInfo) {
        let image = &self.texture;

        let mut width = outer_rect.size.width;
        let mut height = outer_rect.size.height;

        if image.width() != outer_rect.size.width && image.height() != outer_rect.size.height {
            width = outer_rect.size.width.min(width);
            height = outer_rect.size.height.min(height);
            let new_image = image.resize(width, height, imageops::Gaussian);
            self.raw_pixels = new_image.raw_pixels();
            width = new_image.width();
            height = new_image.width();
        }

        let mut frame = Rect {pos : POS_ZERO, size : Size {width : width, height : height}};

        frame.pos.x = ((outer_rect.size.width as f32 * self.gravity.x) - (frame.size.width as f32 * self.gravity.x)) as u32;
        frame.pos.y = ((outer_rect.size.height as f32 * self.gravity.y) - (frame.size.height as f32 * self.gravity.y)) as u32;

        self.frame = frame;

    }

    fn render(&mut self, fixed_rect:&Rect, screen_info:&ScreenInfo, canvas_ptr:*mut u32) {
        render_to_canvas(self.raw_pixels.as_slice().as_ptr() as *mut u32, fixed_rect, &self.frame, screen_info, canvas_ptr);
    }
}
