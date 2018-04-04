extern crate xml;
extern crate uuid;

use std::fs::File;
use std::io::BufReader;
use std::path;
use std::f32;
use xml::reader::{EventReader, XmlEvent};

use color::{Color};
use color;
use dimension::*;
use node;
use scene;
use shape;
use screen_writer::ScreenWriter;
use text;
use texture::{TextureSprite};

pub fn show_scene(scene_path:&str, screen_writer:Box<ScreenWriter>) {

    let path = path::Path::new(scene_path).join("scene.xml");

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let parser = EventReader::new(file);



    let mut scene = scene::Scene::new();
    scene.writer = Some(screen_writer);

    let mut current_keys:Vec<node::NodeKey> = Vec::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name == "scene" {
                    let root_node = process_scene_attributes(&scene, attributes);
                    current_keys.push(root_node.key);
                    scene.set_root_node(root_node);
                }
                else if name.local_name == "box" {
                    let node = process_box_attributes(attributes);
                    let node_key = node.key;
                    if let Some(parent_key) = current_keys.last() {
                        let parent_key = node::Node::deref_node_key(parent_key);
                        scene.add_node(node, parent_key);
                    }
                    current_keys.push(node_key);
                }
                else if name.local_name == "text" {
                    let mut text = String::new();
                    text.push_str("Test");
                    let node = process_text_attributes(attributes, text);
                    let node_key = node.key;
                    if let Some(parent_key) = current_keys.last().clone() {
                        let parent_key = node::Node::deref_node_key(parent_key);
                        scene.add_node(node, parent_key);
                    }
                    current_keys.push(node_key);
                }
                else if name.local_name == "image" {
                    let mut texture = TextureSprite::new_for_texture("image1.png");
                    let node = process_texture_attributes(texture, attributes);
                    let node_key = node.key;
                    if let Some(parent_key) = current_keys.last().clone() {
                        let parent_key = node::Node::deref_node_key(parent_key);
                        scene.add_node(node, parent_key);
                    }
                    current_keys.push(node_key);
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "text" {
                    if let Some(parent_key) = current_keys.last().clone() {
                        if let Some(node) = scene.nodes.get(parent_key) {

                        }
                    }
                }
                current_keys.pop();
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

fn process_scene_attributes<'a>(_scene:&scene::Scene, attributes:Vec<xml::attribute::OwnedAttribute>) -> node::Node<'a> {
    let color = resolve_color_from_attributes(&attributes, color::GRAY);

    let mut box_sprite = shape::RectSprite::new();
    box_sprite.color = color;
    node::Node::new_rect_node(FLOAT_RECT_FULL, box_sprite)
}

fn process_box_attributes<'a>(attributes:Vec<xml::attribute::OwnedAttribute>) -> node::Node<'a> {
    let color = resolve_color_from_attributes(&attributes, color::GRAY);
    let pos = resolve_position_from_attributes(&attributes, FLOAT_POS_ZERO);
    let size = resolve_size_from_attributes(&attributes, FLOAT_SIZE_HALF);
    let anchor_point = resolve_anchor_from_attributes(&attributes, ANCHOR_POINT_CENTER);

    let mut box_sprite = shape::RectSprite::new();
    box_sprite.color = color;
    let mut node = ::Node::new_rect_node(FloatRect{pos:pos, size:size}, box_sprite);
    node.anchor_point = anchor_point;
    node
}

fn process_text_attributes<'a>(attributes:Vec<xml::attribute::OwnedAttribute>, text:String) -> node::Node<'a> {
    let color = resolve_color_from_attributes(&attributes, color::GRAY);
    let pos = resolve_position_from_attributes(&attributes, FLOAT_POS_ZERO);
    let size = resolve_size_from_attributes(&attributes, FLOAT_SIZE_HALF);
    let anchor_point = resolve_anchor_from_attributes(&attributes, ANCHOR_POINT_CENTER);
    let height = resolve_float_from_attributes("height", &attributes, 0.10);
    let text = resolve_text_from_attributes("text", &attributes, String::new());


    let mut text_sprite = text::TextSprite::new_for_text(text.as_ref(), "Arial.ttf");
    text_sprite.height = height;

    let mut node = node::Node::new_text_node(FloatRect{pos:pos, size:size}, text_sprite);
    node.anchor_point = anchor_point;
    node
}

fn process_texture_attributes<'a>(texture:TextureSprite, _attributes:Vec<xml::attribute::OwnedAttribute>) -> node::Node<'a> {
    node::Node::new_texture_node(FloatRect{pos:FloatPos{x:0.0, y:0.0}, size:FloatSize{width:0.5, height:0.5}}, texture)
}

fn resolve_color_from_attributes(attributes:&Vec<xml::attribute::OwnedAttribute>, default:Color) -> Color {
    if let Some(attribute) = attribute_by_name(attributes, "color") {
        let value = &attribute.value;
        if value.starts_with("#") {
            if let Some(color) = color::color_by_hex(value.get(1..).unwrap()) {
                return color;
            }
        }
            else {
                if let Some(color) = color::color_by_name(&value) {
                    return color;
                }
            }
    }
    default
}

fn resolve_position_from_attributes(attributes:&Vec<xml::attribute::OwnedAttribute>, default:FloatPos) -> FloatPos {
    if let Some(attribute) = attribute_by_name(attributes, "pos") {
        let value = &attribute.value;
        let tokens:Vec<&str> = value.split_whitespace().collect();
        if tokens.len() == 2 {
            return FloatPos {
                x : resolve_float_from_value(tokens[0], default.x),
                y : resolve_float_from_value(tokens[1], default.y),
            }
        }
        else if tokens.len() == 1 {
            return FloatPos {
                x : resolve_float_from_value(tokens[0], default.x),
                y : resolve_float_from_value(tokens[0], default.y),
            }
        }
    }
    default
}

fn resolve_size_from_attributes(attributes:&Vec<xml::attribute::OwnedAttribute>, default:FloatSize) -> FloatSize {
    if let Some(attribute) = attribute_by_name(attributes, "size") {
        let value = &attribute.value;
        let tokens:Vec<&str> = value.split_whitespace().collect();
        if tokens.len() == 2 {
            return FloatSize {
                width : resolve_float_from_value(tokens[0], default.width),
                height : resolve_float_from_value(tokens[1], default.height),
            }
        }
        else if tokens.len() == 1 {
            return FloatSize {
                width : resolve_float_from_value(tokens[0], default.width),
                height : resolve_float_from_value(tokens[0], default.height),
            }
        }
    }
    default
}

fn resolve_anchor_from_attributes(attributes:&Vec<xml::attribute::OwnedAttribute>, default:AnchorPoint) -> AnchorPoint {
    if let Some(attribute) = attribute_by_name(attributes, "anchor-point") {
        let value = &attribute.value;
        let tokens:Vec<&str> = value.split_whitespace().collect();
        if tokens.len() == 2 {
            return AnchorPoint {
                x : resolve_float_from_value(tokens[0], default.x),
                y : resolve_float_from_value(tokens[1], default.y),
            }
        }
            else if tokens.len() == 1 {
                return AnchorPoint {
                    x : resolve_float_from_value(tokens[0], default.x),
                    y : resolve_float_from_value(tokens[0], default.y),
                }
            }
    }
    default
}
fn resolve_float_from_attributes(name:&str, attributes:&Vec<xml::attribute::OwnedAttribute>, default:f32) -> f32 {
    if let Some(attribute) = attribute_by_name(attributes, name) {
        let value = &attribute.value;
        return resolve_float_from_value(value, default);
    }
    default
}

fn resolve_text_from_attributes(name:&str, attributes:&Vec<xml::attribute::OwnedAttribute>, default:String) -> String {
    if let Some(attribute) = attribute_by_name(attributes, name) {
        return attribute.value.clone();
    }
    default
}

fn resolve_float_from_value(value:&str, default:f32) -> f32 {
    let value = String::from(value);
    if value.ends_with("%") {
        if let Some(value) = value.get(..(value.len() - 1)) {
            if let Ok(float_value) = value.parse::<f32>() {
                return float_value / 100.0;
            }
        }
    }
    else {
        if let Ok(float_value) = value.parse::<f32>() {
            return float_value;
        }
    }
    default
}

fn attribute_by_name<'a>(attributes:&'a Vec<xml::attribute::OwnedAttribute>, name:&str) -> Option<&'a xml::attribute::OwnedAttribute> {
    attributes.iter().find(|attribute| attribute.name.local_name == name)
}
