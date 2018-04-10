use screen_writer::{ScreenInfo, PIXEL_RGBA};
use dimension::Rect;

pub trait Sprite<'a> {
    fn draw(&mut self, fixed_rect: &Rect, screen_info: &ScreenInfo);
    fn render(
        &mut self,
        parent_node_frame: &Rect,
        node_frame: &Rect,
        clip_to_bounds: bool,
        screen_info: &ScreenInfo,
        canvas_ptr: *mut u32,
    );
}

impl<'a> Sprite<'a> {}
pub fn render_to_canvas(
    raw_pixels_ptr: *const u32,
    parent_node_frame: &Rect,
    outer_frame: &Rect,
    inner_frame: &Rect,
    clip_to_bounds: bool,
    screen_info: &ScreenInfo,
    canvas_ptr: *mut u32,
) {
    let outer_offset = outer_frame.pos.y * screen_info.xres + outer_frame.pos.x;
    let inner_offset = outer_offset + inner_frame.pos.y * screen_info.xres + inner_frame.pos.x;

    for y in 0..inner_frame.size.height {
        let effective_y = y + outer_frame.pos.y + inner_frame.pos.y;
        let inside_screen_y = effective_y >= 0 && effective_y < screen_info.yres;
        let inside_bounds_y = y + inner_frame.pos.y >= 0
            && y + inner_frame.pos.y < outer_frame.size.height
            && (effective_y >= parent_node_frame.pos.y
                && effective_y < (parent_node_frame.pos.y + parent_node_frame.size.height));

        if inside_screen_y && ((clip_to_bounds && inside_bounds_y) || !clip_to_bounds) {
            for x in 0..inner_frame.size.width {
                let effective_x = x + outer_frame.pos.x + inner_frame.pos.x;
                let inside_screen_x = effective_x >= 0 && effective_x < screen_info.xres;
                let inside_bounds_x = x + inner_frame.pos.x >= 0
                    && x + inner_frame.pos.x < outer_frame.size.width
                    && (effective_x >= parent_node_frame.pos.x
                        && effective_x < (parent_node_frame.pos.x + parent_node_frame.size.width));

                if inside_screen_x && ((clip_to_bounds && inside_bounds_x) || !clip_to_bounds) {
                    let dst_offset = inner_offset + y * screen_info.xres + x;
                    let src_offset = y * inner_frame.size.width + x;
                    unsafe {
                        let dst_pixel = *canvas_ptr.offset(dst_offset as isize);
                        let dst_r = ((dst_pixel >> screen_info.pixel_def.red_offset) & 0xFF) as f32;
                        let dst_g =
                            ((dst_pixel >> screen_info.pixel_def.green_offset) & 0xFF) as f32;
                        let dst_b =
                            ((dst_pixel >> screen_info.pixel_def.blue_offset) & 0xFF) as f32;

                        let src_pixel = *raw_pixels_ptr.offset(src_offset as isize); // src_slice_u32[src_offset];

                        let src_r = (src_pixel >> PIXEL_RGBA.red_offset) & 0xFF;
                        let src_g = (src_pixel >> PIXEL_RGBA.green_offset) & 0xFF;
                        let src_b = (src_pixel >> PIXEL_RGBA.blue_offset) & 0xFF;
                        let src_a = (src_pixel >> PIXEL_RGBA.transp_offset) & 0xFF;

                        if src_a as u32 == 0xFF {
                            let new_pixel = (src_r << screen_info.pixel_def.red_offset)
                                | (src_g << screen_info.pixel_def.green_offset)
                                | (src_b << screen_info.pixel_def.blue_offset)
                                | (0xFF << screen_info.pixel_def.transp_offset);

                            *canvas_ptr.offset(dst_offset as isize) = new_pixel;
                        } else if src_a as u32 != 0 {
                            let src_r = src_r as f32;
                            let src_g = src_g as f32;
                            let src_b = src_b as f32;
                            let src_a = src_a as f32;

                            let right_a = src_a / 0xFF as f32;
                            let left_a = 1.0 - right_a;

                            let r = src_r * right_a + dst_r * left_a;
                            let g = src_g * right_a + dst_g * left_a;
                            let b = src_b * right_a + dst_b * left_a;

                            let new_pixel = ((r as u32) << screen_info.pixel_def.red_offset)
                                | ((g as u32) << screen_info.pixel_def.green_offset)
                                | ((b as u32) << screen_info.pixel_def.blue_offset)
                                | (0xFF << screen_info.pixel_def.transp_offset);

                            *canvas_ptr.offset(dst_offset as isize) = new_pixel;
                        } else if screen_info.show_debug_info
                            && (x == 0 || y == 0 || x == inner_frame.size.width - 1
                                || y == inner_frame.size.height - 1)
                        {
                            *canvas_ptr.offset(dst_offset as isize) = 0xFF000000;
                        }
                    }
                }
            }
        }
    }

    if screen_info.show_debug_info {
        for y in 0..outer_frame.size.height {
            let effective_y = y + outer_frame.pos.y;
            if effective_y >= 0 && effective_y < screen_info.yres {
                for x in 0..outer_frame.size.width {
                    let effective_x = x + outer_frame.pos.x;
                    if effective_x >= 0 && effective_x < screen_info.xres {
                        if x == 0 || y == 0 || x == outer_frame.size.width - 1
                            || y == outer_frame.size.height - 1
                        {
                            unsafe {
                                let dst_offset = outer_offset + y * screen_info.xres + x;
                                *canvas_ptr.offset(dst_offset as isize) = 0xFFFFFFFF;
                            }
                        }
                    }
                }
            }
        }
    }
}
