extern crate libc;

use libc::ioctl;
use std::fs::{OpenOptions, File};
use std::os::unix::io::AsRawFd;
use std::io;

const FBIOGET_VSCREENINFO:  libc::c_ulong = 0x4600;
const FBIOPUT_VSCREENINFO:  libc::c_ulong = 0x4601;
const FBIOGET_FSCREENINFO:  libc::c_ulong = 0x4602;

const KDSETMODE:            libc::c_ulong = 0x4B3A;
const KD_TEXT:              libc::c_ulong = 0x00;
const KD_GRAPHICS:          libc::c_ulong = 0x01;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct fb_bitfield {
    pub offset: u32,
    pub length: u32,
    pub msb_right: u32,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct fb_fix_screeninfo {
    pub id: [u8; 16],
    pub smem_start: usize,
    pub smem_len: u32,
    pub fb_type: u32,
    pub type_aux: u32,
    pub visual: u32,
    pub xpanstep: u16,
    pub ypanstep: u16,
    pub ywrapstep: u16,
    pub line_length: u32,
    pub mmio_start: usize,
    pub mmio_len: u32,
    pub accel: u32,
    pub capabilities: u16,
    pub reserved: [u16; 2],
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct fb_var_screeninfo {
    pub xres: u32,
    pub yres: u32,
    pub xres_virtual: u32,
    pub yres_virtual: u32,
    pub xoffset: u32,
    pub yoffset: u32,
    pub bits_per_pixel: u32,
    pub grayscale: u32,
    pub red: fb_bitfield,
    pub green: fb_bitfield,
    pub blue: fb_bitfield,
    pub transp: fb_bitfield,
    pub nonstd: u32,
    pub activate: u32,
    pub height: u32,
    pub width: u32,
    pub accel_flags: u32,
    pub pixclock: u32,
    pub left_margin: u32,
    pub right_margin: u32,
    pub upper_margin: u32,
    pub lower_margin: u32,
    pub hsync_len: u32,
    pub vsync_len: u32,
    pub sync: u32,
    pub vmode: u32,
    pub rotate: u32,
    pub colorspace: u32,
    pub reserved: [u32; 4],
}

impl ::std::default::Default for fb_fix_screeninfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

impl ::std::default::Default for fb_var_screeninfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub fn get_fix_screeninfo(dev: &File) -> Result<fb_fix_screeninfo, io::Error> {
    let mut value: fb_fix_screeninfo = Default::default();
    let result = unsafe {
        ioctl(dev.as_raw_fd(), FBIOGET_FSCREENINFO, &mut value)
    };
    match result {
        -1 => Err(io::Error::new(io::ErrorKind::Other, "get_fix_screeninfo - Ioctl failed")),
        _ => Ok(value),
    }
}

pub fn get_var_screeninfo(dev: &File) -> Result<fb_var_screeninfo, io::Error> {
    let mut value: fb_var_screeninfo = Default::default();
    let result = unsafe {
        ioctl(dev.as_raw_fd(), FBIOGET_VSCREENINFO, &mut value)
    };
    match result {
        -1 => Err(io::Error::new(io::ErrorKind::Other, "get_var_screeninfo - Ioctl failed")),
        _ => Ok(value),
    }
}

pub fn put_var_screeninfo(dev: &File, vinfo: &fb_var_screeninfo) -> Result<(), io::Error> {
    let new_vinfo = vinfo.clone();
    let result = unsafe {
//        new_vinfo.activate = 256;
        ioctl(dev.as_raw_fd(), FBIOPUT_VSCREENINFO, &new_vinfo)
    };
    match result {
        -1 => Err(io::Error::new(io::ErrorKind::Other, "put_var_screeninfo - Ioctl failed")),
        _ => Ok(()),
    }
}

pub fn set_graphics_mode() -> Result<(), io::Error> {
    let result = unsafe {
        let tty_dev = OpenOptions::new().read(true).write(true).open("/dev/tty")?;
        ioctl(tty_dev.as_raw_fd(), KDSETMODE, KD_GRAPHICS)
    };
    match result {
        -1 => Err(io::Error::new(io::ErrorKind::Other, "set_graphics_mode - Ioctl failed")),
        _ => Ok(()),
    }
}

pub fn set_text_mode() -> Result<(), io::Error> {
    let result = unsafe {
        let tty_dev = OpenOptions::new().read(true).write(true).open("/dev/tty")?;
        ioctl(tty_dev.as_raw_fd(), KDSETMODE, KD_TEXT)
    };
    match result {
        -1 => Err(io::Error::new(io::ErrorKind::Other, "set_text_mode - Ioctl failed")),
        _ => Ok(()),
    }
}
