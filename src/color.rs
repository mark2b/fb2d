use std::mem;

pub struct Color {
    pub rgba : [u8; 4],
    pub rgba_as_u32 : u32,
}

impl Color {
    pub fn red() -> Color {
        Color { rgba : [0xFF, 0x00, 0x00, 0xFF], rgba_as_u32 : 0xFF0000FF}
    }

    pub fn green() -> Color {
        Color { rgba : [0x00, 0xFF, 0x00, 0xFF], rgba_as_u32 : 0xFF00FF00}
    }

    pub fn blue() -> Color {
        Color { rgba : [0x00, 0x00, 0xFF, 0xFF], rgba_as_u32 : 0xFFFF0000}
    }

    pub fn white() -> Color {
        Color { rgba : [0xFF, 0xFF, 0xFF, 0xFF], rgba_as_u32 : 0xFFFFFFFF}
    }

    pub fn as_rgba(&self) -> u32 {
        unsafe {
            mem::transmute::<[u8;4], u32>(self.rgba)
        }
    }
    pub fn as_rgba_with_alpha(&self, alpha:u8) -> u32 {
        let mut rgba = self.rgba;
        rgba[3] = alpha;
        unsafe {
            mem::transmute::<[u8;4], u32>(rgba)
        }
    }
}
