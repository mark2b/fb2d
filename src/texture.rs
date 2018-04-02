extern crate image;

use std::fs;
use std::io::{BufReader};
use image::{GenericImage, DynamicImage, imageops};

use dimension::{FixRect, FixSize, FIX_POS_ZERO};
use screen_writer::{ScreenInfo};
use sprite::{Sprite, render_to_canvas};

pub struct TextureSprite {
    texture : DynamicImage,
    size : FixSize,
    raw_pixels : Vec<u8>,
}

impl TextureSprite {

    pub fn new_for_texture(filename: &str) -> TextureSprite {

        let file: fs::File = fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let load_result = image::load(reader, image::PNG).unwrap();

        TextureSprite {
            raw_pixels : load_result.raw_pixels(),
            size : FixSize {
                width : load_result.width(),
                height : load_result.height(),
            },
            texture : load_result,
        }
    }
}

impl Sprite for TextureSprite {

    fn draw(&mut self, fixed_rect:&FixRect, _screen_info:&ScreenInfo) {
        let image = &self.texture;

        if image.width() != fixed_rect.size.width && image.height() != fixed_rect.size.height {
            let new_image = image.resize(fixed_rect.size.width, fixed_rect.size.height, imageops::Gaussian);
            self.raw_pixels = new_image.raw_pixels();
            self.size = FixSize {
                width : new_image.width(),
                height : new_image.height(),
            };
        }
    }

    fn render(&mut self, fixed_rect:&FixRect, screen_info:&ScreenInfo, canvas_ptr:*mut u32) {
        let src_slice_ptr_u32 = self.raw_pixels.as_slice().as_ptr() as *mut u32;
        let inner_frame = FixRect { pos: FIX_POS_ZERO, size: FixSize { width: self.size.width, height : self.size.height}};
        render_to_canvas(src_slice_ptr_u32, fixed_rect, &inner_frame, screen_info, canvas_ptr);
    }
}
