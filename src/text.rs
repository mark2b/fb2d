extern crate rusttype;

use std::cmp;
use std::fs;
use std::u32;
use std::io::Read;
use rusttype::{Font, FontCollection, point, PositionedGlyph};

use color::*;
use color;
use dimension::*;
use screen_writer::*;
use sprite::{Sprite, render_to_canvas};


pub struct TextSprite {
    pub color: Color,
    pub gravity : Gravity,
    pub scale : Scale,
    pub height : f32,
    pub text : String,
    pub font : Font<'static>,
    frame: Rect,
    raw_pixels : Vec<u32>,
}

impl TextSprite {

    pub fn new() -> TextSprite {

        let font = FontCollection::from_bytes(get_default_font_data()).unwrap().into_font().unwrap();

        TextSprite {
            color : color::WHITE,
            gravity : GRAVITY_CENTER,
            font : font,
            height : 1.0,
            scale : SCALE_SINGLE,
            text : String::new(),
            frame: RECT_ZERO,
            raw_pixels : Vec::new(),
        }
    }
}

impl<'a> Sprite<'a> for TextSprite {
    fn draw(&mut self, outer_rect:&Rect, _screen_info:&ScreenInfo) {
        let height = outer_rect.size.height as f32 * self.height;
        let scale = rusttype::Scale { x: height * self.scale.x, y: height * self.scale.y};
        let v_metrics = self.font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);
        let glyphs: Vec<PositionedGlyph> = self.font.layout(self.text.as_ref(), scale, offset).collect();

        let text_frame = {
            let mut min_x = u32::MAX;
            let mut max_x = u32::MIN;
            let mut min_y = u32::MAX;
            let mut max_y = u32::MIN;
            for g in glyphs.clone() {
                if let Some(bb) = g.pixel_bounding_box() {
                    min_x = cmp::min(min_x, bb.min.x as u32);
                    max_x = cmp::max(max_x, bb.max.x as u32);
                    min_y = cmp::min(min_y, bb.min.y as u32);
                    max_y = cmp::max(max_y, bb.max.y as u32);
                }
            }
            Rect {pos : Pos {x : min_x, y : min_y} , size : Size {width : max_x - min_x, height : max_y - min_y}}
        };

        let mut frame = Rect {pos : POS_ZERO, size : text_frame.size};

        let buffer_size = (frame.size.width * frame.size.height) as usize;

        self.raw_pixels = vec![0; buffer_size];
        let raw_pixels_ptr = self.raw_pixels.as_mut_slice().as_mut_ptr();

        frame.pos.x = ((outer_rect.size.width as f32 * self.gravity.x) - (frame.size.width as f32 * self.gravity.x)) as u32;
        frame.pos.y = ((outer_rect.size.height as f32 * self.gravity.y) - (frame.size.height as f32 * self.gravity.y)) as u32;


        for g in glyphs.clone() {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|gx, gy, gv| {
                    let gx = gx as i32 + bb.min.x - text_frame.pos.x as i32;
                    let gy = gy as i32 + bb.min.y - text_frame.pos.y as i32;
                    let a = (gv * 255.0) as u8;
                    let dst_offset = (gy * frame.size.width as i32 + gx) as isize;
                    unsafe {
                        let pixel = color::WHITE.color_with_alpha(a);
                        *raw_pixels_ptr.offset(dst_offset) = pixel;
                    }
                })
            }
        }
        self.frame = frame;
    }

    fn render(&mut self, fixed_rect: &Rect, screen_info: &ScreenInfo, canvas_ptr:*mut u32) {
        render_to_canvas(self.raw_pixels.as_ptr(), fixed_rect, &self.frame, screen_info, canvas_ptr);
    }
}

fn get_default_font_data() -> Vec<u8> {
    Vec::from(include_bytes!("default.ttf") as &[u8])
}

