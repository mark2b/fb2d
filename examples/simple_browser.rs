extern crate ctrlc;
extern crate fb2d;

use std::process;
use fb2d::version;

fn main() {
    println!("commit: {} {}", version::commit_date(), version::short_sha());

    ctrlc::set_handler(move || {
        fb2d::set_text_mode();
        process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    match run() {
        Ok(_) => {}
        Err(e) => println!("{:?}", e),
    };
}

fn run() -> Result<(), String> {
    match fb2d::scene::Scene::new_from_bundle("assets/scene1") {
        Ok(mut scene) => {
            #[cfg(not(target_os = "linux"))]
            let mut fb = fb2d::screen_writer_for_png("frame_buffer.png", 1920, 1080)?;
            #[cfg(target_os = "linux")]
            let mut fb = fb2d::screen_writer_for_framebuffer("/dev/fb0")?;
            #[cfg(target_os = "linux")]
            fb2d::set_graphics_mode();
            fb.screen_info.show_debug_info = true;

            scene.writer = Some(Box::new(fb));

            scene.run();
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }

    Ok(())
}
