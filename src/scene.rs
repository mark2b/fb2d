use std::cell;
use std::rc;
use std::collections::*;
use std::sync;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::time;
use std::thread::sleep;
use screen_writer::{ScreenInfo, ScreenWriter};

use dimension::*;
use node::*;

pub struct SceneState {
    running: bool,
}

type OnEveryFrame = fn(state: SceneState) -> SceneState;

pub struct Scene<'a> {
    pub writer: Option<Box<ScreenWriter>>,
    pub canvas_buffer: cell::RefCell<Vec<u32>>,
    pub nodes: HashMap<NodeKey, cell::RefCell<Node<'a>>>,
    pub hierarchy: HashMap<NodeKey, cell::RefCell<Vec<NodeKey>>>,
    root_node_key: NodeKey,
    fps: u32,
    dirty: bool,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene {
            fps: 60,
            dirty: true,
            writer: None,
            canvas_buffer: cell::RefCell::new(vec![]),
            nodes: HashMap::new(),
            hierarchy: HashMap::new(),
            root_node_key: EMPTY_NODE_KEY,
        }
    }

    pub fn set_root_node(&mut self, node: Node<'a>) {
        self.root_node_key = node.key;
        self.nodes.insert(node.key, cell::RefCell::new(node));
    }

    pub fn add_node(&mut self, node: Node<'a>, to_key: NodeKey) {
        let mut found = false;
        if let Some(key_cell) = self.hierarchy.get(&to_key) {
            let mut children_keys = key_cell.borrow_mut();
            children_keys.push(node.key);
            found = true;
        }

        if !found {
            self.hierarchy
                .insert(to_key, cell::RefCell::new(vec![node.key; 1]));
        }
        self.nodes.insert(node.key, cell::RefCell::new(node));
    }

    fn layout(&self, screen_info: &ScreenInfo) {
        let frame_rect = Rect {
            size: Size {
                width: screen_info.xres,
                height: screen_info.yres,
            },
            pos: POS_ZERO,
        };

        if let Some(ref root_node) = self.nodes.get(&self.root_node_key) {
            let mut root_node_mut = root_node.borrow_mut();
            root_node_mut.layout(frame_rect, screen_info);

            if let Some(key_cell) = self.hierarchy.get(&root_node_mut.key) {
                let mut children_keys = key_cell.borrow_mut();
                self.layout_nodes(
                    &frame_rect,
                    &root_node_mut.anchor_point,
                    &children_keys,
                    screen_info,
                );
            }
        }
    }

    fn layout_nodes(
        &self,
        parent_node_rect: &Rect,
        parent_acnhor_point: &AnchorPoint,
        nodes_keys: &Vec<NodeKey>,
        screen_info: &ScreenInfo,
    ) {
        for node_key in nodes_keys {
            if let Some(ref node) = self.nodes.get(node_key) {
                let mut node_mut = node.borrow_mut();
                let frame_rect =
                    node_mut.fix_rect_for_parent_fix_rect(parent_node_rect, &parent_acnhor_point);

                node_mut.layout(frame_rect, screen_info);

                if let Some(key_cell) = self.hierarchy.get(&node_mut.key) {
                    let mut children_keys = key_cell.borrow_mut();
                    self.layout_nodes(
                        &frame_rect,
                        &node_mut.anchor_point,
                        &children_keys,
                        screen_info,
                    );
                }
            }
        }
    }

    fn draw_root_node(&self, screen_info: &ScreenInfo) {
        if let Some(ref root_node) = self.nodes.get(&self.root_node_key) {
            let mut root_node_mut = root_node.borrow_mut();
            root_node_mut.draw_if_need(screen_info);

            if let Some(key_cell) = self.hierarchy.get(&root_node_mut.key) {
                let mut children_keys = key_cell.borrow_mut();
                self.draw_nodes(&children_keys, screen_info);
            }
        }
    }

    fn draw_nodes(&self, nodes_keys: &Vec<NodeKey>, screen_info: &ScreenInfo) {
        for node_key in nodes_keys {
            if let Some(ref node) = self.nodes.get(node_key) {
                let mut node_mut = node.borrow_mut();
                node_mut.draw_if_need(screen_info);

                if let Some(key_cell) = self.hierarchy.get(&node_mut.key) {
                    let mut children_keys = key_cell.borrow_mut();
                    self.draw_nodes(&children_keys, screen_info);
                }
            }
        }
    }

    fn render_root_node(&self, screen_info: &ScreenInfo) {
        if let Some(ref root_node) = self.nodes.get(&self.root_node_key) {
            let mut root_node_mut = root_node.borrow_mut();
            root_node_mut.draw_if_need(screen_info);
            let root_node_frame = root_node_mut.frame;
            root_node_mut.render(
                &root_node_frame,
                screen_info,
                self.canvas_buffer.borrow_mut().as_mut_ptr() as *mut u32,
            );

            if let Some(key_cell) = self.hierarchy.get(&root_node_mut.key) {
                let mut children_keys = key_cell.borrow_mut();
                self.render_nodes(&root_node_frame, &children_keys, screen_info);
            }
        }
    }

    fn render_nodes(
        &self,
        parent_node_frame: &Rect,
        nodes_keys: &Vec<NodeKey>,
        screen_info: &ScreenInfo,
    ) {
        for node_key in nodes_keys {
            if let Some(ref node) = self.nodes.get(node_key) {
                let mut node_mut = node.borrow_mut();

                node_mut.render(
                    parent_node_frame,
                    screen_info,
                    self.canvas_buffer.borrow_mut().as_mut_ptr() as *mut u32,
                );

                if let Some(key_cell) = self.hierarchy.get(&node_mut.key) {
                    let mut children_keys = key_cell.borrow_mut();
                    self.render_nodes(&node_mut.frame, &children_keys, screen_info);
                }
            }
        }
    }

    fn render_frame(&self, screen_info: &ScreenInfo) {
        self.draw_root_node(screen_info);
        self.render_root_node(screen_info);
        // FIX IT
        if let Some(ref writer) = self.writer {
            writer.write(self.canvas_buffer.borrow_mut().to_vec());
        }
    }

    pub fn run_with_state(&mut self, on_every_frame_function: OnEveryFrame) {
        if let Some(ref writer) = self.writer {
            let screen_info = writer.get_screen_info();
            self.canvas_buffer =
                cell::RefCell::new(vec![0xFF; writer.get_screen_info().screen_size]);

            let frame_duration = time::Duration::from_millis((1000 / self.fps) as u64);
            let mut counter = 0;
            self.layout(screen_info);

            let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

            let mut state = SceneState { running: true };

            loop {
                state = on_every_frame_function(state);

                if !state.running {
                    break;
                }

                let start_time = time::SystemTime::now();
                if self.dirty {
                    self.render_frame(screen_info);
                    self.dirty = false;
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
    }

    pub fn run(&mut self) {
        if let Some(ref writer) = self.writer {
            let screen_info = writer.get_screen_info();
            self.canvas_buffer =
                cell::RefCell::new(vec![0xFF; writer.get_screen_info().screen_size]);

            let frame_duration = time::Duration::from_millis((1000 / self.fps) as u64);
            let mut counter = 0;
            self.layout(screen_info);
            loop {
                let start_time = time::SystemTime::now();
                if self.dirty {
                    self.render_frame(screen_info);
                    self.dirty = false;
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
    }

    pub fn run_once(&mut self) {
        if let Some(ref writer) = self.writer {
            let screen_info = writer.get_screen_info();
            self.canvas_buffer =
                cell::RefCell::new(vec![0xFF; writer.get_screen_info().screen_size]);
            self.layout(screen_info);
            let start_time = time::SystemTime::now();

            self.render_frame(screen_info);

            let end_time = time::SystemTime::now();
            let duration = end_time.duration_since(start_time).unwrap();
            println!("duration:{:?}", duration);
        }
    }
}
