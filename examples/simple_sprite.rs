extern crate ctrlc;
extern crate fb2d;

use std::process;

use fb2d::{Node, Scene};

use fb2d::ScreenWriterError;
use fb2d::{RectSprite, TextSprite, TextureSprite};
use fb2d::*;

fn main() {
    ctrlc::set_handler(move || {
        fb2d::set_text_mode();
        process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    match run() {
        Ok(_) => {}
        Err(e) => println!("{:?}", e),
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

    let mut scene = Scene::new();

    let background_sprite = RectSprite::new(fb2d::Color::blue());
    let background_node = Node::new_rect_node(FLOAT_RECT_FULL, background_sprite);

    let sprite1 = RectSprite::new(fb2d::Color::green());
    let mut node1 = Node::new_rect_node(
        FloatRect {
            pos: FLOAT_POS_ZERO,
            size: FLOAT_SIZE_HALF,
        },
        sprite1,
    );
    node1.anchor_point = ANCHOR_POINT_TOP_LEFT;

    let sprite2 = TextureSprite::new_for_texture("mmm.png");
    let mut node2 = Node::new_texture_node(
        FloatRect {
            pos: FLOAT_POS_ZERO,
            size: FloatSize {
                width: 0.7,
                height: 0.7,
            },
        },
        sprite2,
    );
    node2.anchor_point = ANCHOR_POINT_CENTER;

    let mut sprite3 = TextSprite::new_for_text("Hello, World !!!", "Arial.ttf");
    sprite3.gravity = GRAVITY_CENTER;
    sprite3.height = 0.2;
    let node3 = Node::new_text_node(
        FloatRect {
            pos: FLOAT_POS_ZERO,
            size: FLOAT_SIZE_FULL,
        },
        sprite3,
    );

    scene.add_node(node2, node1.key);
    scene.add_node(node1, background_node.key);
    scene.add_node(node3, background_node.key);
    scene.set_root_node(background_node);

    scene.writer = Some(Box::new(fb));

    scene.run();

    Ok(())
}
