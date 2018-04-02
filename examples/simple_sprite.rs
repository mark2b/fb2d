extern crate fb2d;
extern crate ctrlc;

use std::process;
use std::cell;

use fb2d::{Scene, Node};

use fb2d::{ScreenWriterError};
use fb2d::{RectSprite, TextureSprite, TextSprite};
use fb2d::*;

fn main() {
    ctrlc::set_handler(move || {
        fb2d::set_text_mode();
        process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    match run() {
        Ok(_) => {},
        Err(e) => println!("{:?}", e)
    };
}

fn run() -> Result<(), ScreenWriterError> {
    #[cfg(feature = "simulator")]
    let mut fb = fb2d::screen_writer_for_png("frame_buffer.png", 1920, 1080)?;
    #[cfg(not(feature = "simulator"))]
    let mut fb = fb2d::screen_writer_for_framebuffer("/dev/fb0")?;

    #[cfg(not(feature = "simulator"))]
    fb2d::set_graphics_mode();
    fb.screen_info.show_debug_info = true;


    let mut sprite1 = RectSprite::new(fb2d::Color::green());
    let mut node1 = Node::new_rect_node(FloatRect{pos:FLOAT_POS_ZERO, size:FLOAT_SIZE_HALF}, sprite1);
    node1.anchor_point = ANCHOR_POINT_TOP_LEFT;


    let mut sprite2=  TextureSprite::new_for_texture("mmm.png");
    let mut node2 = Node::new_texture(FloatRect{pos:FLOAT_POS_ZERO, size: FloatSize {width : 0.7, height : 0.7}}, sprite2);
    node2.anchor_point = ANCHOR_POINT_CENTER;


    &node1.add_node(node2);

    let mut sprite3=  TextSprite::new_for_text("Hello, World !!!", "DejaVuSans.ttf");
    sprite3.gravity = GRAVITY_CENTER;

    let mut node3 = Node::new_text(FloatRect{pos:FLOAT_POS_ZERO, size:FLOAT_SIZE_FULL}, sprite3);


    let mut background_sprite = RectSprite::new(fb2d::Color::blue());
    let mut background_node = Node::new_rect_node(FLOAT_RECT_FULL, background_sprite);

    &background_node.add_node(node1);
    &background_node.add_node(node3);



    let mut scene = Scene::new();
    scene.root_node = cell::Cell::new(Some(background_node));
    scene.writer = Some(Box::new(fb));


    #[cfg(feature = "simulator")]
    scene.run_once();
    #[cfg(not(feature = "simulator"))]
    scene.run();

    Ok(())

}