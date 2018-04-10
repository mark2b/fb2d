extern crate uuid;

use std::ptr;

use screen_writer::{ScreenInfo};
use text::*;
use texture::*;
use dimension::*;
use shape::*;
use sprite::*;

pub struct Node<'a> {
    pub key : NodeKey,
    pub tag : String,
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

    pub fn fix_rect_for_parent_fix_rect(&self, parent_node_rect:&Rect, parent_anchor_point:&AnchorPoint) -> Rect {

        let mut node_width = self.float_frame.size.width * parent_node_rect.size.width as f32;
        let mut node_height = self.float_frame.size.height * parent_node_rect.size.height as f32;

        let parent_node_x = parent_node_rect.pos.x as f32 + parent_node_rect.size.width as f32 * parent_anchor_point.x;
        let parent_node_y = parent_node_rect.pos.y as f32 + parent_node_rect.size.height as f32 * parent_anchor_point.y;

        let node_pos_x = parent_node_x + self.float_frame.pos.x * parent_node_rect.size.width as f32;
        let node_pos_y = parent_node_y + self.float_frame.pos.y * parent_node_rect.size.height as f32;

        let mut node_x = node_pos_x - node_width * self.anchor_point.x;
        let mut node_y = node_pos_y - node_height * self.anchor_point.y;

        Rect {
            pos : Pos {
                x : node_x as i32,
                y : node_y as i32,
            },
            size : Size {
                width : node_width as i32,
                height : node_height as i32,
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
            tag : String::new(),
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
             tag : String::new(),
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
            tag : String::new(),
            float_frame : float_frame,
            anchor_point: ANCHOR_POINT_CENTER,
            frame: RECT_ZERO,
            need_draw: true,
            sprite: Box::new(sprite),
        }
    }
}

