extern crate rusttype;

use std::cmp;
use std::fs;
use std::u32;
use std::io::Read;
use rusttype::{Font, FontCollection, Scale, point, PositionedGlyph};

use color::*;
use dimension::*;
use screen_writer::*;
use sprite::{Sprite, render_to_canvas};

pub struct TextSprite<'a> {
    pub color: Color,
    text : String,
    font : Font<'a>,
    rect : FixRect,
    raw_pixels : Vec<u32>,
}

impl<'a> TextSprite<'a> {

    pub fn new_for_text(text: &str, font_name: &str) -> TextSprite<'a> {
        let mut file: fs::File = fs::File::open(font_name).unwrap();

        let mut font_buffer = Vec::new();
        file.read_to_end(&mut font_buffer).unwrap();

        let font = FontCollection::from_bytes(font_buffer).unwrap().into_font().unwrap();

        TextSprite {
            color : Color::white(),
            font : font,
            text : String::from(text),
            rect : FIX_RECT_ZERO,
            raw_pixels : Vec::new(),
        }
    }
}

impl<'a> Sprite for TextSprite<'a> {
    fn draw(&mut self, fixed_rect:&FixRect, _screen_info:&ScreenInfo) {
        self.rect = *fixed_rect;

        let height = 48.0;
        let scale = Scale { x: height * 1.0, y: height };
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
            FixRect {pos : FixPos {x : min_x, y : min_y} , size : FixSize {width : max_x - min_x, height : max_y - min_y}}
        };

        let frame = FixRect {pos : FIX_POS_ZERO, size : text_frame.size};

        let buffer_size = (frame.size.width * frame.size.height) as usize;

        self.raw_pixels = vec![0; buffer_size];
        let raw_pixels_ptr = self.raw_pixels.as_mut_slice().as_mut_ptr();

        self.rect = frame;

        for g in glyphs.clone() {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|gx, gy, gv| {
                    let gx = gx as i32 + bb.min.x - text_frame.pos.x as i32;
                    let gy = gy as i32 + bb.min.y - text_frame.pos.y as i32;
                    let a = (gv * 255.0) as u8;
                    if gv > 0.1 {
                        let dst_offset = (gy * frame.size.width as i32 + gx) as isize;
                        unsafe {
                            let pixel = Color::white().as_rgba_with_alpha( a);
                            *raw_pixels_ptr.offset(dst_offset) = pixel;
                        }
                    }
                })
            }
        }
    }

    fn render(&mut self, fixed_rect: &FixRect, screen_info: &ScreenInfo, canvas_ptr:*mut u32) {
        let src_slice_ptr_u32 = self.raw_pixels.as_ptr();
        render_to_canvas(src_slice_ptr_u32, fixed_rect, &self.rect, screen_info, canvas_ptr);
    }
}

