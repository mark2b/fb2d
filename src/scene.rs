use std::time;
use std::thread::sleep;
use screen_writer::{ScreenWriter};

use color::{Color};
use dimension::{FixRect, FixSize, FixPos};
use node::Node;

pub struct Scene {
    pub writer: Option<Box<ScreenWriter>>,
    pub root_node:Node,
    fps:u32,
    dirty:bool,
    pub canvas_buffer: Vec<u32>,
}

impl Scene {

    pub fn new() -> Scene {
        let root_node = Node::new_rect_node(1.0, 1.0, Color::blue());

        Scene {
            root_node : root_node,
            fps : 60,
            dirty : true,
            writer : None,
            canvas_buffer : vec![]
        }
    }

    fn layout(&mut self) {
        if self.writer.is_some() {
            let writer = self.writer.as_ref().unwrap().as_ref();
            let screen_info = writer.get_screen_info();
            let frame_rect = FixRect {
                size : FixSize {
                    width : screen_info.xres,
                    height : screen_info.yres},
                pos : FixPos {x : 0, y : 0}

            };
            self.root_node.layout(frame_rect, writer.get_screen_info());
        }
    }

    fn render_frame(&mut self) {

        if self.writer.is_some() {
            let writer = self.writer.as_ref().unwrap().as_ref();
            if self.canvas_buffer.len() == 0 {
                self.canvas_buffer = vec![0xFF; writer.get_screen_info().screen_size];
            }

            let screen_info = writer.get_screen_info();
            if self.root_node.need_draw {
                self.root_node.draw(screen_info)
            }

            self.root_node.render(screen_info, self.canvas_buffer.as_mut_ptr() as *mut u32);
            // FIX IT
            writer.write(self.canvas_buffer.clone());
        }
    }

    pub fn run(&mut self) {
        let frame_duration = time::Duration::from_millis((1000 / self.fps) as u64);
        let mut counter = 0;
        self.layout();
        loop {
            let start_time = time::SystemTime::now();
            if self.dirty {
                self.render_frame();
//                self.dirty = false;
            }
            let end_time = time::SystemTime::now();

            let duration = end_time.duration_since(start_time).unwrap();
            if frame_duration > duration {
                sleep(frame_duration - duration);
            }
            counter += 1;

            if counter % 100 == 0 {
                println!("duration:{:?}", duration);
            }
        }
    }

    pub fn run_once(&mut self) {
        self.layout();
        let start_time = time::SystemTime::now();
        self.render_frame();
        let end_time = time::SystemTime::now();
        let duration = end_time.duration_since(start_time).unwrap();
        println!("duration:{:?}", duration);
    }
}
