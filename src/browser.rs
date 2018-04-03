extern crate xml;
extern crate uuid;

use std::fs::File;
use std::io::BufReader;
use std::path;
use xml::reader::{EventReader, XmlEvent};

use color;
use dimension::*;
use node;
use scene;
use shape;
use screen_writer::ScreenWriter;

pub fn show_scene(scene_path:&str, screen_writer:Box<ScreenWriter>) {

    let path = path::Path::new(scene_path).join("scene.xml");

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let parser = EventReader::new(file);



    let mut scene = scene::Scene::new();
    scene.writer = Some(screen_writer);

    let mut parent_key = [0u8;16];


    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                println!("+{}", name);
                if name.local_name == "scene" {
                    let root_node = process_scene_attributes(&scene, attributes);
                    parent_key = root_node.key;
                    scene.set_root_node(root_node);
                }
                else if name.local_name == "rect" {
                    let mut shape = shape::RectSprite::new(color::Color::red());
                    let mut node = process_shape_attributes(shape, attributes);
                    scene.add_node(node, parent_key)

                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                println!("-{}", name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    scene.run();
}

fn process_scene_attributes<'a>(_scene:&scene::Scene, _attributes:Vec<xml::attribute::OwnedAttribute>) -> node::Node<'a> {
    let rectangular = shape::RectSprite::new(color::Color::blue());
    node::Node::new_rect_node(FLOAT_RECT_FULL, rectangular)
}

fn process_shape_attributes<'a>(shape:shape::RectSprite, _attributes:Vec<xml::attribute::OwnedAttribute>) -> node::Node<'a> {
    node::Node::new_rect_node(FloatRect{pos:FloatPos{x:0.3, y:0.3}, size:FloatSize{width:0.5, height:0.5}}, shape)
}