extern crate fb2d;
extern crate ctrlc;

use std::process;

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

    fb.screen_info.show_debug_info = true;

//    fb2d::browser::show_scene("assets/scene1", Box::new(fb));

    let mut scene = fb2d::scene::Scene::new_from_bundle("assets/scene1");
    scene.writer = Some(Box::new(fb));

    scene.run();


    Ok(())
}