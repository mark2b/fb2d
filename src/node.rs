extern crate uuid;

use std::ptr;

use screen_writer::{ScreenInfo};
use text::*;
use texture::*;
use dimension::*;
use shape::*;
use sprite::Sprite;

pub struct Node<'a> {
    pub key : NodeKey,
    pub float_frame : FloatRect,
    pub anchor_point:AnchorPoint,
    pub frame : Rect,
    pub need_draw: bool,
    sprite: Box<Sprite<'a>>,
}

pub type NodeKey = [u8;16];
pub const EMPTY_NODE_KEY : NodeKey = [0u8;16];

impl<'a, 'b : 'a> Node<'a> {

    pub fn deref_node_key(node_key_ref:&NodeKey) -> NodeKey {
        let mut key = [0u8;16];
        unsafe {
            ptr::copy(node_key_ref.as_ptr(), key.as_mut_ptr(), 16);
        }
        key
    }

    pub fn fix_rect_for_parent_fix_rect(&self, parent_node_rect:&Rect) -> Rect {
        let node_width = (self.float_frame.size.width * (parent_node_rect.size.width as f32)) as u32;
        let node_height = (self.float_frame.size.height * (parent_node_rect.size.height as f32)) as u32;

//        println!("node_width, node_height {:?} {:?}", node_width, node_height);
//        println!("parent_node_rect {:?}", parent_node_rect);

        let half_parent_width = (parent_node_rect.size.width >> 1) as f32;
        let half_parent_height = (parent_node_rect.size.height >> 1) as f32;

        Rect {
            pos : Pos {
                x : parent_node_rect.pos.x + (((parent_node_rect.size.width - node_width) as f32 * self.anchor_point.x + (self.float_frame.pos.x * half_parent_width)) as u32),
                y : parent_node_rect.pos.y + (((parent_node_rect.size.height - node_height) as f32 * self.anchor_point.y + (self.float_frame.pos.y * half_parent_height)) as u32),
            },
            size : Size {
                width : node_width,
                height : node_height,
            },
        }
    }

    pub fn layout(&mut self, frame: Rect, _screen_info:&ScreenInfo) {
        self.frame = frame;
    }

    pub fn draw_if_need(&mut self, screen_info:&ScreenInfo) {
        if self.need_draw {
            self.sprite.draw(&self.frame, screen_info);
            self.need_draw = false;
        }
    }

    pub fn render(&mut self, screen_info:&ScreenInfo, canvas_ptr:*mut u32) {
        self.sprite.render(&self.frame, screen_info, canvas_ptr);
    }

    fn generate_key() -> [u8;16] {
        let uuid_key = uuid::Uuid::new_v4();
        let mut key = [0u8;16];
        unsafe {
            ptr::copy(uuid_key.as_bytes().as_ptr(), key.as_mut_ptr(), 16);
        }
        key
    }

    pub fn new_rect_node(float_frame:FloatRect, sprite:RectSprite) -> Node<'a> {
        Node {
            key : Self::generate_key(),
            float_frame : float_frame,
            anchor_point: ANCHOR_POINT_CENTER,
            frame: RECT_ZERO,
            need_draw: true,
            sprite: Box::new(sprite),
        }
    }

    pub fn new_text_node(float_frame:FloatRect, sprite:TextSprite) -> Node<'a> {
         Node {
             key : Self::generate_key(),
             float_frame : float_frame,
             anchor_point: ANCHOR_POINT_CENTER,
             frame: RECT_ZERO,
             need_draw: true,
             sprite: Box::new(sprite),
        }
    }

    pub fn new_texture_node(float_frame:FloatRect, sprite:TextureSprite) -> Node<'a> {
        Node {
            key : Self::generate_key(),
            float_frame : float_frame,
            anchor_point: ANCHOR_POINT_CENTER,
            frame: RECT_ZERO,
            need_draw: true,
            sprite: Box::new(sprite),
        }
    }
}

