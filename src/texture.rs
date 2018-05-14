extern crate image;

use std::fs;
use std::io;
use image::{GenericImage, DynamicImage, imageops};

use dimension::*;
use screen_writer::{ScreenInfo};
use sprite::{Sprite, render_to_canvas};

pub struct TextureSprite {
    pub gravity : Gravity,
    pub filter : imageops::FilterType,
    texture : Option<DynamicImage>,
    raw_pixels : Vec<u8>,
    frame : Rect,
}

impl TextureSprite {

    pub fn new() -> TextureSprite {

        TextureSprite {
            raw_pixels : Vec::new(),
            gravity : GRAVITY_CENTER,
            texture : None,
            filter : imageops::Triangle,
            frame : RECT_ZERO,
        }
    }

    fn load_image(filename: &str) -> DynamicImage {
        let file: fs::File = fs::File::open(filename).unwrap();
        let reader = io::BufReader::new(file);
        if filename.to_lowercase().ends_with(".png") {
            image::load(reader, image::PNG).unwrap()
        } else if filename.to_lowercase().ends_with(".png") {
            image::load(reader, image::JPEG).unwrap()
        } else {
            image::load(reader, image::PNG).unwrap()
        }
    }

    pub fn new_for_texture(filename: &str) -> TextureSprite {
        let load_result = TextureSprite::load_image(filename);

        TextureSprite {
            raw_pixels : load_result.raw_pixels(),
            filter : imageops::Triangle,
            gravity : GRAVITY_CENTER,
            texture : Some(load_result),
            frame : RECT_ZERO,
        }
    }

    pub fn set_texture_filename(&mut self, filename: &str) {
        let load_result = TextureSprite::load_image(filename);
        self.texture = Some(load_result);
    }
}

impl<'a> Sprite<'a> for TextureSprite {

    fn draw(&mut self, outer_rect:&Rect, _screen_info:&ScreenInfo) {
        if let Some(ref image) = self.texture {

            let frame_aspect = outer_rect.size.width as f32 / outer_rect.size.height as f32;
            let image_aspect = image.width() as f32 / image.height() as f32;

            let mut width: i32;
            let mut height: i32;

            if frame_aspect < image_aspect {
                width = outer_rect.size.width;
                height = (width as f32 / image_aspect) as i32;
            } else {
                height = outer_rect.size.height;
                width = (height as f32 * image_aspect) as i32;
            }

            let new_image = image.resize(width as u32, height as u32, self.filter);
            self.raw_pixels = new_image.raw_pixels();
            width = new_image.width() as i32;
            height = new_image.height() as i32;

            let mut frame = Rect {pos : POS_ZERO, size : Size {width : width, height : height}};
            frame.pos.x = ((outer_rect.size.width as f32 * self.gravity.x) - (frame.size.width as f32 * self.gravity.x)) as i32;
            frame.pos.y = ((outer_rect.size.height as f32 * self.gravity.y) - (frame.size.height as f32 * self.gravity.y)) as i32;
            self.frame = frame;
        }
    }

    fn render(&mut self, parent_node_frame:&Rect, node_frame:&Rect, clip_to_bounds:bool, screen_info:&ScreenInfo, canvas_ptr:*mut u32) {
        render_to_canvas(self.raw_pixels.as_slice().as_ptr() as *mut u32, parent_node_frame, node_frame, &self.frame, clip_to_bounds, screen_info, canvas_ptr);
    }
}
