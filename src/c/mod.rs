mod c;

pub use self::c::{
    fb_fix_screeninfo,
    fb_var_screeninfo,
    fb_bitfield,
    get_fix_screeninfo,
    get_var_screeninfo,
    put_var_screeninfo,
    set_graphics_mode,
    set_text_mode,
};
