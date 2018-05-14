extern crate log;
extern crate tempdir;
extern crate uuid;
extern crate xml;
extern crate image;

use std::fs;
use std::f32;
use xml::reader::{EventReader, XmlEvent};
use image::{imageops};

use color::Color;
use color;
use dimension::*;
use node;
use resource;
use scene::Scene;
use shape;
use text;
use texture::TextureSprite;

impl<'a> Scene<'a> {
    pub fn new_from_bundle(path: &str) -> Result<Scene<'a>, String> {
        let mut scene_bundle = resource::SceneBundle::new(path);

        scene_bundle.open()?;

        let target_path = scene_bundle.target_path();

        let scene_xml_file_path = &target_path.join("scene.xml");
        match fs::File::open(scene_xml_file_path) {
            Ok(scene_xml_file) => Self::parse_scene_xml(scene_xml_file, &scene_bundle),
            Err(e) => Err(format!("{} {}", line!(), e)),
        }
    }

    pub fn parse_scene_xml(
        scene_xml_file: fs::File,
        scene_bundle: &resource::SceneBundle,
    ) -> Result<Scene<'a>, String> {
        let parser = EventReader::new(scene_xml_file);

        let mut scene = Scene::new();

        let mut current_keys: Vec<node::NodeKey> = Vec::new();

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.local_name == "scene" {
                        let root_node = process_scene_attributes(&scene, attributes);
                        current_keys.push(root_node.key);
                        scene.set_root_node(root_node);
                    } else if name.local_name == "box" {
                        let node = process_box_attributes(attributes);
                        let node_key = node.key;
                        if let Some(parent_key) = current_keys.last() {
                            let parent_key = node::Node::deref_node_key(parent_key);
                            scene.add_node(node, parent_key);
                        }
                        current_keys.push(node_key);
                    } else if name.local_name == "text" {
                        let node = process_text_attributes(attributes, &scene_bundle);
                        let node_key = node.key;
                        if let Some(parent_key) = current_keys.last().clone() {
                            let parent_key = node::Node::deref_node_key(parent_key);
                            scene.add_node(node, parent_key);
                        }
                        current_keys.push(node_key);
                    } else if name.local_name == "image" {
                        let node = process_texture_attributes(attributes, &scene_bundle);
                        let node_key = node.key;
                        if let Some(parent_key) = current_keys.last().clone() {
                            let parent_key = node::Node::deref_node_key(parent_key);
                            scene.add_node(node, parent_key);
                        }
                        current_keys.push(node_key);
                    }
                }
                Ok(XmlEvent::EndElement { .. }) => {
                    current_keys.pop();
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
        Ok(scene)
    }
}

fn process_scene_attributes<'a>(
    _scene: &Scene,
    attributes: Vec<xml::attribute::OwnedAttribute>,
) -> node::Node<'a> {
    let tag = resolve_text_from_attributes("tag", &attributes, String::new());
    let anchor_point = resolve_anchor_from_attributes(&attributes, ANCHOR_POINT_CENTER);
    let alpha = resolve_float_from_attributes("alpha", &attributes, 1.0);
    let color =
        resolve_color_from_attributes(&attributes, color::GRAY).color_with_alpha_float(alpha);

    let mut box_sprite = shape::RectSprite::new();
    box_sprite.color = color;

    let mut node = node::Node::new_rect_node(FLOAT_RECT_FULL, box_sprite);
    node.anchor_point = anchor_point;
    node.tag = tag;
    node
}

fn process_box_attributes<'a>(attributes: Vec<xml::attribute::OwnedAttribute>) -> node::Node<'a> {
    let visible = resolve_bool_from_attributes("visible", &attributes, true);
    let tag = resolve_text_from_attributes("tag", &attributes, String::new());
    let alpha = resolve_float_from_attributes("alpha", &attributes, 1.0);
    let color =
        resolve_color_from_attributes(&attributes, color::GRAY).color_with_alpha_float(alpha);
    let pos = resolve_position_from_attributes(&attributes, FLOAT_POS_ZERO);
    let size = resolve_size_from_attributes(&attributes, FLOAT_SIZE_HALF);
    let anchor_point = resolve_anchor_from_attributes(&attributes, ANCHOR_POINT_CENTER);
    let clip_to_bounds = resolve_bool_from_attributes("clip_to_bounds", &attributes, false);

    let mut box_sprite = shape::RectSprite::new();
    box_sprite.color = color;
    let mut node = ::Node::new_rect_node(
        FloatRect {
            pos: pos,
            size: size,
        },
        box_sprite,
    );
    node.anchor_point = anchor_point;
    node.tag = tag;
    node.clip_to_bounds = clip_to_bounds;
    node.visible = visible;
    node
}

fn process_text_attributes<'a>(
    attributes: Vec<xml::attribute::OwnedAttribute>,
    scene_bundle: &resource::SceneBundle,
) -> node::Node<'a> {
    let visible = resolve_bool_from_attributes("visible", &attributes, true);
    let tag = resolve_text_from_attributes("tag", &attributes, String::new());
    let alpha = resolve_float_from_attributes("alpha", &attributes, 1.0);
    let color =
        resolve_color_from_attributes(&attributes, color::GRAY).color_with_alpha_float(alpha);
    let pos = resolve_position_from_attributes(&attributes, FLOAT_POS_ZERO);
    let size = resolve_size_from_attributes(&attributes, FLOAT_SIZE_HALF);
    let anchor_point = resolve_anchor_from_attributes(&attributes, ANCHOR_POINT_CENTER);
    let height = resolve_float_from_attributes("height", &attributes, 1.0);
    let text = resolve_text_from_attributes("text", &attributes, String::new());
    let font_filename = resolve_text_from_attributes("font", &attributes, String::new());
    let clip_to_bounds = resolve_bool_from_attributes("clip_to_bounds", &attributes, false);

    let mut text_sprite = text::TextSprite::new();
    text_sprite.height = height;
    text_sprite.text = text;
    text_sprite.color = color;

    if font_filename.len() > 0 {
        let font_filename_path = scene_bundle.target_path().join(font_filename);

        match fs::File::open(font_filename_path) {
            Ok(mut font_file) => text_sprite.set_font_file(&mut font_file),
            Err(e) => println!("{:?}", e),
        }
    }

    let mut node = node::Node::new_text_node(
        FloatRect {
            pos: pos,
            size: size,
        },
        text_sprite,
    );
    node.anchor_point = anchor_point;
    node.tag = tag;
    node.clip_to_bounds = clip_to_bounds;
    node.visible = visible;
    node
}

fn process_texture_attributes<'a>(
    attributes: Vec<xml::attribute::OwnedAttribute>,
    scene_bundle: &resource::SceneBundle,
) -> node::Node<'a> {
    let visible = resolve_bool_from_attributes("visible", &attributes, true);
    let tag = resolve_text_from_attributes("tag", &attributes, String::new());
    let pos = resolve_position_from_attributes(&attributes, FLOAT_POS_ZERO);
    let size = resolve_size_from_attributes(&attributes, FLOAT_SIZE_HALF);
    let anchor_point = resolve_anchor_from_attributes(&attributes, ANCHOR_POINT_CENTER);
    let texture_filename = resolve_text_from_attributes("image", &attributes, String::new());
    let clip_to_bounds = resolve_bool_from_attributes("clip_to_bounds", &attributes, false);
    let filter = resolve_text_from_attributes("filter", &attributes, String::from("triangle")).to_lowercase();

    let mut texture_sprite = TextureSprite::new();

    let texture_filename_path = scene_bundle.target_path().join(texture_filename);
    texture_sprite.set_texture_filename(texture_filename_path.into_os_string().into_string().unwrap().as_ref());

    texture_sprite.filter = {
        if filter == "nearest" {
            imageops::Nearest
        } else if filter == "triangle" {
            imageops::Triangle
        } else if filter == "catmullRom" {
            imageops::CatmullRom
        } else if filter == "gaussian" {
            imageops::Gaussian
        } else if filter == "lanczos3" {
            imageops::Lanczos3
        } else {
            imageops::Triangle
        }
    };


    let mut node = node::Node::new_texture_node(
        FloatRect {
            pos: pos,
            size: size,
        },
        texture_sprite,
    );
    node.anchor_point = anchor_point;
    node.tag = tag;
    node.clip_to_bounds = clip_to_bounds;
    node.visible = visible;
    node
}

fn resolve_color_from_attributes(
    attributes: &Vec<xml::attribute::OwnedAttribute>,
    default: Color,
) -> Color {
    if let Some(attribute) = attribute_by_name(attributes, "color") {
        let value = &attribute.value;
        if value.starts_with("#") {
            if let Some(color) = color::color_by_hex(value.get(1..).unwrap()) {
                return color;
            }
        } else {
            if let Some(color) = color::color_by_name(&value) {
                return color;
            }
        }
    }
    default
}

fn resolve_position_from_attributes(
    attributes: &Vec<xml::attribute::OwnedAttribute>,
    default: FloatPos,
) -> FloatPos {
    if let Some(attribute) = attribute_by_name(attributes, "pos") {
        let value = &attribute.value;
        let tokens: Vec<&str> = value.split_whitespace().collect();
        if tokens.len() == 2 {
            return FloatPos {
                x: resolve_float_from_value(tokens[0], default.x),
                y: resolve_float_from_value(tokens[1], default.y),
            };
        } else if tokens.len() == 1 {
            return FloatPos {
                x: resolve_float_from_value(tokens[0], default.x),
                y: resolve_float_from_value(tokens[0], default.y),
            };
        }
    }
    default
}

fn resolve_size_from_attributes(
    attributes: &Vec<xml::attribute::OwnedAttribute>,
    default: FloatSize,
) -> FloatSize {
    if let Some(attribute) = attribute_by_name(attributes, "size") {
        let value = &attribute.value;
        let tokens: Vec<&str> = value.split_whitespace().collect();
        if tokens.len() == 2 {
            return FloatSize {
                width: resolve_float_from_value(tokens[0], default.width),
                height: resolve_float_from_value(tokens[1], default.height),
            };
        } else if tokens.len() == 1 {
            return FloatSize {
                width: resolve_float_from_value(tokens[0], default.width),
                height: resolve_float_from_value(tokens[0], default.height),
            };
        }
    }
    default
}

fn resolve_anchor_from_attributes(
    attributes: &Vec<xml::attribute::OwnedAttribute>,
    default: AnchorPoint,
) -> AnchorPoint {
    if let Some(attribute) = attribute_by_name(attributes, "anchor-point") {
        let value = &attribute.value;
        let tokens: Vec<&str> = value.split_whitespace().collect();
        if tokens.len() == 2 {
            return AnchorPoint {
                x: resolve_float_from_value(tokens[0], default.x),
                y: resolve_float_from_value(tokens[1], default.y),
            };
        } else if tokens.len() == 1 {
            return AnchorPoint {
                x: resolve_float_from_value(tokens[0], default.x),
                y: resolve_float_from_value(tokens[0], default.y),
            };
        }
    }
    default
}
fn resolve_float_from_attributes(
    name: &str,
    attributes: &Vec<xml::attribute::OwnedAttribute>,
    default: f32,
) -> f32 {
    if let Some(attribute) = attribute_by_name(attributes, name) {
        let value = &attribute.value;
        return resolve_float_from_value(value, default);
    }
    default
}

fn resolve_text_from_attributes(
    name: &str,
    attributes: &Vec<xml::attribute::OwnedAttribute>,
    default: String,
) -> String {
    if let Some(attribute) = attribute_by_name(attributes, name) {
        return attribute.value.clone();
    }
    default
}

fn resolve_bool_from_attributes(
    name: &str,
    attributes: &Vec<xml::attribute::OwnedAttribute>,
    default: bool,
) -> bool {
    if let Some(attribute) = attribute_by_name(attributes, name) {
        let value = attribute.value.to_lowercase();
        return value == "true" || value == "yes";
    }
    default
}

fn resolve_float_from_value(value: &str, default: f32) -> f32 {
    let value = String::from(value);
    if value.ends_with("%") {
        if let Some(value) = value.get(..(value.len() - 1)) {
            if let Ok(float_value) = value.parse::<f32>() {
                return float_value / 100.0;
            }
        }
    } else {
        if let Ok(float_value) = value.parse::<f32>() {
            return float_value;
        }
    }
    default
}

fn attribute_by_name<'a>(
    attributes: &'a Vec<xml::attribute::OwnedAttribute>,
    name: &str,
) -> Option<&'a xml::attribute::OwnedAttribute> {
    attributes
        .iter()
        .find(|attribute| attribute.name.local_name == name)
}
