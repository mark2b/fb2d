use std::cell;

use screen_writer::{ScreenInfo};
use text::*;
use texture::*;
use dimension::*;
use shape::*;
use sprite::Sprite;

pub struct Node<'a> {
    pub float_frame : FloatRect,
    pub anchor_point:AnchorPoint,
    pub frame : Rect,
    pub need_draw: bool,
    children: Vec<cell::RefCell<Node<'a>>>,
    sprite: Box<Sprite<'a>>,
}

impl<'a, 'b : 'a> Node<'a> {
    fn fix_rect_for_parent_fix_rect(&self, parent_node_rect:&Rect) -> Rect {
        let node_width = (self.float_frame.size.width * (parent_node_rect.size.width as f32)) as u32;
        let node_height = (self.float_frame.size.height * (parent_node_rect.size.height as f32)) as u32;

        let half_parent_width = (parent_node_rect.size.width >> 1) as f32;
        let half_parent_height = (parent_node_rect.size.height >> 1) as f32;

        Rect {
            pos : Pos {
                x : ((parent_node_rect.size.width - node_width) as f32 * self.anchor_point.x + (self.float_frame.pos.x * half_parent_width)) as u32,
                y : ((parent_node_rect.size.height - node_height) as f32 * self.anchor_point.y + (self.float_frame.pos.y * half_parent_height)) as u32,
            },
            size : Size {
                width : node_width,
                height : node_height,
            },
        }
    }

    pub fn layout(&mut self, frame: Rect, screen_info:&ScreenInfo) {
        self.frame = frame;

        for child in &self.children {
            let mut child_node = child.borrow_mut();
            let child_node_frame = child_node.fix_rect_for_parent_fix_rect(&self.frame);
            child_node.layout(child_node_frame, screen_info);
        }
    }

    pub fn draw(&mut self, screen_info:&ScreenInfo) {
        self.sprite.draw(&self.frame, screen_info);
        self.need_draw = false;

        for child in &self.children {
            let mut child_node = child.borrow_mut();
            child_node.draw(screen_info);
        }
    }

    pub fn render(&mut self, screen_info:&ScreenInfo, canvas_ptr:*mut u32) {
        self.sprite.render(&self.frame, screen_info, canvas_ptr);

        for child in &self.children {
            let mut child_node = child.borrow_mut();
            child_node.render(screen_info, canvas_ptr);
        }
    }

    pub fn add_node(&mut self, node:Node<'a>) {
        self.children.push(cell::RefCell::new(node));
    }

    pub fn new_rect_node(float_frame:FloatRect, sprite:RectSprite) -> Node<'a> {
        Node {
            float_frame : float_frame,
            anchor_point: ANCHOR_POINT_CENTER,
            children: vec![],
            frame: RECT_ZERO,
            need_draw: true,
            sprite: Box::new(sprite),
        }
    }

    pub fn new_text(float_frame:FloatRect, sprite:TextSprite) -> Node<'a> {
         Node {
            float_frame : float_frame,
            anchor_point: ANCHOR_POINT_CENTER,
            children: vec![],
             frame: RECT_ZERO,
            need_draw: true,
            sprite: Box::new(sprite),
        }
    }

    pub fn new_texture(float_frame:FloatRect, sprite:TextureSprite) -> Node<'a> {
        Node {
            float_frame : float_frame,
            anchor_point: ANCHOR_POINT_CENTER,
            children: vec![],
            frame: RECT_ZERO,
            need_draw: true,
            sprite: Box::new(sprite),
        }
    }
}

