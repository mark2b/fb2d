use std::cell;
use std::rc::Rc;
use std::borrow::BorrowMut;

use screen_writer::{ScreenInfo};
use text::*;
use texture::*;
use color::{Color};
use dimension::*;
use shape::*;
use sprite::Sprite;

pub struct Node<'a> {
    pub size:Size,
    pub pos:Pos,
    pub anchor_point:AnchorPoint,
    pub fix_rect : FixRect,
    pub need_draw: bool,
    children: Vec<cell::RefCell<Node<'a>>>,
    sprite: Box<Sprite<'a>>,
}

impl<'a, 'b : 'a> Node<'a> {
    fn fix_rect_for_parent_fix_rect(&self, parent_fixed_rect:&FixRect) -> FixRect {
        let node_fix_width = (self.size.width * (parent_fixed_rect.size.width as f32)) as u32;
        let node_fix_height = (self.size.height * (parent_fixed_rect.size.height as f32)) as u32;

        let half_parent_width = (parent_fixed_rect.size.width >> 1) as f32;
        let half_parent_height = (parent_fixed_rect.size.height >> 1) as f32;

        FixRect {
            pos : FixPos {
                x : ((parent_fixed_rect.size.width - node_fix_width) as f32 * self.anchor_point.x + (self.pos.x * half_parent_width)) as u32,
                y : ((parent_fixed_rect.size.height - node_fix_height) as f32 * self.anchor_point.y + (self.pos.y * half_parent_height)) as u32,
            },
            size : FixSize {
                width : node_fix_width,
                height : node_fix_height,
            },
        }
    }

    pub fn layout(&mut self, fix_rect:FixRect, screen_info:&ScreenInfo) {
        self.fix_rect = fix_rect;

        for child in &self.children {
            let mut child_node = child.borrow_mut();
            let child_node_fix_rect = child_node.fix_rect_for_parent_fix_rect(&self.fix_rect);
            child_node.layout(child_node_fix_rect, screen_info);
        }
    }

    pub fn draw(&mut self, screen_info:&ScreenInfo) {
        self.sprite.draw(&self.fix_rect, screen_info);
        self.need_draw = false;

        for child in &self.children {
            let mut child_node = child.borrow_mut();
            child_node.draw(screen_info);
        }
    }

    pub fn render(&mut self, screen_info:&ScreenInfo, canvas_ptr:*mut u32) {
        self.sprite.render(&self.fix_rect, screen_info, canvas_ptr);

        for child in &self.children {
            let mut child_node = child.borrow_mut();
            child_node.render(screen_info, canvas_ptr);
        }
    }

    pub fn add_node(&mut self, node:Node<'a>) {
        self.children.push(cell::RefCell::new(node));
    }

//    pub fn new_rect_node(width:f32, height: f32, color: Color) -> Node<'a> {
//        Node {
//            size: Size {width : width, height : height},
//            pos: Pos {x : 0.0, y : 0.0},
//            anchor_point: AnchorPoint {x: 0.5,  y: 0.5},
//            children: vec![],
//            fix_rect: FIX_RECT_ZERO,
//            need_draw: true,
//            sprite: Rc::new(cell::RefCell::new(RectSprite::new(color)))
//        }
//    }
//
//    pub fn node_from_texture(width:f32, height: f32, filename: &str) -> Node<'a> {
//        Node {
//            size: Size {width : width, height : height},
//            pos: Pos {x : 0.0, y : 0.0},
//            anchor_point: AnchorPoint {x: 0.5,  y: 0.5},
//            children: vec![],
//            fix_rect: FIX_RECT_ZERO,
//            need_draw: true,
//            sprite: Rc::new(cell::RefCell::new(TextureSprite::new_for_texture(filename)))
//        }
//    }
//
//    pub fn node_from_text(width:f32, height: f32, text: &str, fontname: &str) -> Node<'a> {
//        Node {
//            size: Size {width : width, height : height},
//            pos: Pos {x : 0.0, y : 0.0},
//            anchor_point: AnchorPoint {x: 0.5,  y: 0.5},
//            children: vec![],
//            fix_rect: FIX_RECT_ZERO,
//            need_draw: true,
//            sprite: Rc::new(cell::RefCell::new(TextSprite::new_for_text(text, fontname)))
//        }
//    }
    pub fn new_rect(frame:FloatRect, sprite:RectSprite) -> Node<'a> {
        Node {
            size: frame.size,
            pos: frame.pos,
            anchor_point: AnchorPoint { x: 0.5, y: 0.5 },
            children: vec![],
            fix_rect: FIX_RECT_ZERO,
            need_draw: true,
            sprite: Box::new(sprite),
        }
    }

    pub fn new_text(frame:FloatRect, sprite:TextSprite) -> Node<'a> {
         Node {
             size: frame.size,
             pos: frame.pos,
            anchor_point: AnchorPoint { x: 0.5, y: 0.5 },
            children: vec![],
            fix_rect: FIX_RECT_ZERO,
            need_draw: true,
            sprite: Box::new(sprite),
        }
    }

    pub fn new_texture(frame:FloatRect, sprite:TextureSprite) -> Node<'a> {
        Node {
            size: frame.size,
            pos: frame.pos,
            anchor_point: AnchorPoint { x: 0.5, y: 0.5 },
            children: vec![],
            fix_rect: FIX_RECT_ZERO,
            need_draw: true,
            sprite: Box::new(sprite),
        }
    }
}

