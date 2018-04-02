use screen_writer::{ScreenInfo, PIXEL_RGBA};
use dimension::{FixRect};


pub trait Sprite<'a> {
    fn draw(&mut self, fixed_rect:&FixRect, screen_info:&ScreenInfo);
    fn render(&mut self, fixed_rect:&FixRect, screen_info:&ScreenInfo, canvas_ptr:*mut u32);
}

impl<'a> Sprite<'a> {

}
pub fn render_to_canvas(raw_pixels_ptr:*const u32, outer_frame:&FixRect, inner_frame:&FixRect, screen_info:&ScreenInfo, canvas_ptr:*mut u32) {

    let delta_width =  outer_frame.size.width - inner_frame.size.width;
    let delta_height =  outer_frame.size.height - inner_frame.size.height;

    let anchor_point_x = 0.5f32;
    let anchor_point_y = 0.5f32;

    let dst_x_offset = (delta_width as f32 * anchor_point_x) as u32;
    let dst_y_offset = (delta_height as f32  * anchor_point_y) as u32;

    let start_offset = outer_frame.pos.y * screen_info.xres + outer_frame.pos.x;

    for y in 0..inner_frame.size.height {
        if y + outer_frame.pos.y >= screen_info.yres {
            break;
        }
        for x in 0..inner_frame.size.width {
            if x + outer_frame.pos.x >= screen_info.xres {
                break;
            }
            let dst_offset = start_offset + (y + dst_y_offset) * screen_info.xres + (x + dst_x_offset);
            let src_offset = y * inner_frame.size.width + x;
            unsafe {
                let dst_pixel = *canvas_ptr.offset(dst_offset as isize);
                let dst_r = ((dst_pixel >> screen_info.pixel_def.red_offset) & 0xFF) as f32;
                let dst_g = ((dst_pixel >> screen_info.pixel_def.green_offset) & 0xFF) as f32;
                let dst_b = ((dst_pixel >> screen_info.pixel_def.blue_offset) & 0xFF) as f32;

                let src_pixel = *raw_pixels_ptr.offset(src_offset as isize);// src_slice_u32[src_offset];

                let src_r = (src_pixel >> PIXEL_RGBA.red_offset) & 0xFF;
                let src_g = (src_pixel >> PIXEL_RGBA.green_offset) & 0xFF;
                let src_b = (src_pixel >> PIXEL_RGBA.blue_offset) & 0xFF;
                let src_a = (src_pixel >> PIXEL_RGBA.transp_offset) & 0xFF;

                if src_a as u32 == 0xFF {
                    let new_pixel =
                        (src_r << screen_info.pixel_def.red_offset) |
                            (src_g << screen_info.pixel_def.green_offset) |
                            (src_b << screen_info.pixel_def.blue_offset) |
                            (0xFF << screen_info.pixel_def.transp_offset);

                    *canvas_ptr.offset(dst_offset as isize) = new_pixel;
                }
                    else {
                        let src_r = src_r as f32;
                        let src_g = src_g as f32;
                        let src_b = src_b as f32;
                        let src_a = src_a as f32;

                        let right_a = src_a / 0xFF as f32;
                        let left_a = 1.0 - right_a;

                        let r = src_r * right_a + dst_r * left_a;
                        let g = src_g * right_a + dst_g * left_a;
                        let b = src_b * right_a + dst_b * left_a;

                        let new_pixel =
                            ((r as u32) << screen_info.pixel_def.red_offset) |
                            ((g as u32) << screen_info.pixel_def.green_offset) |
                            ((b as u32) << screen_info.pixel_def.blue_offset) |
                                (0xFF << screen_info.pixel_def.transp_offset);

//                        let unit_a = src_a / 0xFF as f32;
//                        let new_pixel =
//                            (((unit_a * src_r + (1.0 - unit_a) * dst_r) as u32) << screen_info.pixel_def.red_offset) |
//                                (((unit_a * src_g + (1.0 - unit_a) * dst_g) as u32) << screen_info.pixel_def.green_offset) |
//                                (((unit_a * src_b + (1.0 - unit_a) * dst_b) as u32) << screen_info.pixel_def.blue_offset) |
//                                (0xFF << screen_info.pixel_def.transp_offset);

                        *canvas_ptr.offset(dst_offset as isize) = new_pixel;
                    }
            }
        }
    }
}
