use std::mem;
use std::i64;
use std::u32;


#[derive(Debug, Copy, Clone)]
pub struct Color(pub u32);

pub struct PredefinedColor {
    pub name : &'static str,
    pub color : Color,
}

pub const GRAY:Color    = Color(0xFF808080);
pub const GREEN:Color   = Color(0xFF008000);
pub const MAROON:Color  = Color(0xFF000080);
pub const SILVER:Color  = Color(0xFFC0C0C0);
pub const WHITE:Color   = Color(0xFFFFFFFF);
pub const YELLOW:Color  = Color(0xFF00FFFF);
pub const RED:Color     = Color(0xFF0000FF);
pub const BLACK:Color   = Color(0xFF000000);
pub const ACUA:Color    = Color(0xFFFFFF00);
pub const FUCHSIA:Color = Color(0xFFFFFF00);
pub const LIME:Color    = Color(0xFF00FF00);
pub const BLUE:Color    = Color(0xFFFF0000);
pub const NAVY:Color    = Color(0xFFFF8000);
pub const OLIVE:Color   = Color(0xFF008080);
pub const PIRPLE:Color  = Color(0xFF800080);
pub const TEAL:Color    = Color(0xFF808000);


static PREDEFINED_COLORS:[PredefinedColor;16] = [
    PredefinedColor { name : "black", color : BLACK},
    PredefinedColor { name : "white", color : WHITE},
    PredefinedColor { name : "red", color : RED},
    PredefinedColor { name : "green", color : GREEN},
    PredefinedColor { name : "blue", color : BLUE},
    PredefinedColor { name : "yellow", color : YELLOW},
    PredefinedColor { name : "gray", color : GRAY},
    PredefinedColor { name : "silver", color : SILVER},
    PredefinedColor { name : "lime", color : LIME},
    PredefinedColor { name : "maroon", color : MAROON},
    PredefinedColor { name : "acua", color : ACUA},
    PredefinedColor { name : "fuchsia", color : FUCHSIA},
    PredefinedColor { name : "navi", color : NAVY},
    PredefinedColor { name : "olive", color : OLIVE},
    PredefinedColor { name : "pirple", color : PIRPLE},
    PredefinedColor { name : "teal", color : TEAL},
];

impl Color {

    pub fn color_with_alpha(&self, alpha:u8) -> Color {
        let Color(c) = *self;
        Color((c & 0x00FFFFFF) | ((alpha as u32) << 24))
    }
    pub fn color_with_alpha_float(&self, alpha:f32) -> Color {
        return self.color_with_alpha((0xFF as f32 * alpha) as u8);
    }
}


pub fn color_by_name(name:&str) -> Option<Color> {

    if let Some(color_def) = PREDEFINED_COLORS.iter().find (|&color_def| color_def.name == name) {
        return Some(color_def.color);
    }
    None
}

pub fn color_by_hex(hex:&str) -> Option<Color> {
    let add_alpha_ff = hex.len() < 8;
    if let Ok(mut color_value) = u32::from_str_radix(&hex, 16) {
        let r = (color_value & 0x00FF0000) >> 16;
        let g = (color_value & 0x0000FF00);
        let b = (color_value & 0x000000FF) << 16;
        if add_alpha_ff {
            color_value |= 0xFF000000;
        }
        let rgb_value = r | g | b | if add_alpha_ff { 0xFF000000 } else  {color_value & 0xFF000000};
        return Some(Color(rgb_value));
    }
    None
}
