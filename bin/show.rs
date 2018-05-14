extern crate clap;
extern crate ctrlc;
extern crate fb2d;

use std::process;
use clap::{App, Arg};

fn main() {
    let matches = App::new("fb2d-show")
        .version("1.0")
        .author("Mark B. <mark2b@gmail.com>")
        .about("Shows scene on linux framebuffer.")
        .arg(
            Arg::with_name("INPUT")
                .help("Scene bundle file or directory to show")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    ctrlc::set_handler(move || {
        fb2d::set_text_mode();
        process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    match run(matches.value_of("INPUT").unwrap()) {
        Ok(_) => {}
        Err(e) => println!("{:?}", e),
    };
}

fn run(input: &str) -> Result<(), String> {
    match fb2d::scene::Scene::new_from_bundle(input) {
        Ok(mut scene) => {
            #[cfg(not(target_os = "linux"))]
            let mut fb = fb2d::screen_writer_for_png("frame_buffer.png", 1920, 1080)?;
            #[cfg(target_os = "linux")]
            let mut fb = fb2d::screen_writer_for_framebuffer("/dev/fb0")?;
            #[cfg(target_os = "linux")]
            fb2d::set_graphics_mode();
            //            fb.screen_info.show_debug_info = true;

            scene.writer = Some(Box::new(fb));
            #[cfg(not(target_os = "linux"))]
            scene.run_once();
            #[cfg(target_os = "linux")]
            scene.run();
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }

    Ok(())
}
