extern crate fb2d;
extern crate ctrlc;

use std::process;

use fb2d::{Scene, Node, Color};

use fb2d::{ScreenWriterError};

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
    let fb = fb2d::screen_writer_for_png("frame_buffer.png", 1920, 1080)?;
    #[cfg(not(feature = "simulator"))]
    let mut fb = fb2d::screen_writer_for_framebuffer("/dev/fb0")?;

    #[cfg(not(feature = "simulator"))]
    fb2d::set_graphics_mode();

    let mut scene = Scene::new();
    scene.writer = Some(Box::new(fb));

    let mut node1 = Node::new_rect_node(0.5, 0.5, Color::green());
//    node1.size.height = 0.3;
    node1.anchor_point.x = 0.0;
    node1.anchor_point.y = 0.0;

    scene.root_node.add_node(node1);


    let node2 = Node::node_from_texture(0.5, 0.5, "mmm.png");
//    node2.pos.x = 0.0;
//    node2.pos.y = 0.0;
//    node2.anchorPoint.x = 0.0;
//    node2.anchorPoint.y = 0.0;
    scene.root_node.add_node(node2);

    let node3 = Node::node_from_text(1.0, 0.5, "Hello, World !!!", "DejaVuSans.ttf");
    scene.root_node.add_node(node3);

    #[cfg(feature = "simulator")]
    scene.run_once();
    #[cfg(not(feature = "simulator"))]
    scene.run();

    Ok(())

}