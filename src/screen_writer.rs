extern crate libc;
extern crate memmap;
extern crate image;

use memmap::{Mmap, Protection};
use std::convert;
use std::fmt;
use std::error::Error;
use std::fs::{OpenOptions, File};
use std::error;
use std::io;
use std::ptr;
use std::rc;
use std::cell;
use std::vec::Vec;
use c;

#[derive(Debug)]
pub struct ScreenInfo {
    pub xres:u32,
    pub yres:u32,
    pub screen_size:usize,
    pub pixel_def:PixelDef,
    pub show_debug_info:bool,
}

#[derive(Debug)]
pub struct PixelDef {
    pub bits_per_pixel:u32,
    pub bytes_per_pixel:u32,
    pub red_offset: u32,
    pub green_offset: u32,
    pub blue_offset: u32,
    pub transp_offset: u32,
}

pub fn set_graphics_mode() {
    match c::set_graphics_mode() {
        Ok(_) => {},
        Err(e) => println!("{:?}", e)
    };
}

pub fn set_text_mode() {
    match c::set_text_mode() {
        Ok(_) => {},
        Err(e) => println!("{:?}", e)
    };
}

pub fn screen_writer_for_framebuffer(devname:&str) -> Result<FrameBuffer, ScreenWriterError>  {
    FrameBuffer::new(devname)
}

pub fn screen_writer_for_png(filename:&str, width:u32, height:u32) -> Result<FrameBufferSimulatorPNG, ScreenWriterError>  {
    FrameBufferSimulatorPNG::new(filename, width, height)
}

pub static PIXEL_RGBA:PixelDef = PixelDef{bits_per_pixel : 32, bytes_per_pixel : 4, red_offset : 0, green_offset : 8, blue_offset : 16, transp_offset : 24};

pub trait ScreenWriter {
    fn get_screen_info(&self) -> &ScreenInfo;
    fn write(&self, page_buffer : Vec<u32>);
}

impl ScreenWriter {

}

impl<'a> ScreenWriter for FrameBufferSimulatorPNG<'a> {

    fn get_screen_info(&self) -> &ScreenInfo {
        &self.screen_info
    }

    fn write(&self, raw_pixels : Vec<u32>) {
        let screen_info = self.get_screen_info();
        let raw_pixels_u8_size = raw_pixels.len() << 2; // * 4
        let mut raw_pixels_u8 = vec![0u8;raw_pixels_u8_size];
        let mut offset_u8 = 0;
        for pixel in raw_pixels {
            raw_pixels_u8[offset_u8] = (pixel >> PIXEL_RGBA.red_offset) as u8;
            raw_pixels_u8[offset_u8 + 1] = (pixel >> PIXEL_RGBA.green_offset) as u8;
            raw_pixels_u8[offset_u8 + 2] = (pixel >> PIXEL_RGBA.blue_offset) as u8;
            raw_pixels_u8[offset_u8 + 3] = (pixel >> PIXEL_RGBA.transp_offset) as u8;
            offset_u8 += 4;
        }
        let im = image::RgbaImage::from_raw(screen_info.xres, screen_info.yres, raw_pixels_u8).unwrap();
        im.save(self.file_name).unwrap();
    }
}

impl<'a> ScreenWriter for FrameBuffer {

    fn get_screen_info(&self) -> &ScreenInfo {
        &self.screen_info
    }

    fn write(&self, raw_pixels : Vec<u32>) {
        unsafe {
            let to_ptr = self.screen_buffer.as_ref().borrow_mut().mut_ptr() as *mut u32;
            ptr::copy(raw_pixels.as_slice().as_ptr(), to_ptr, (self.screen_info.xres * self.screen_info.yres) as usize);
        }
    }
}

#[derive(Debug)]
pub struct FrameBuffer {
    pub screen_info:ScreenInfo,
    pub dev: File,
    pub fix_screen_info: c::fb_fix_screeninfo,
    pub var_screen_info: c::fb_var_screeninfo,
    pub screen_buffer:  rc::Rc<cell::RefCell<Mmap>>,
}

impl FrameBuffer {
    fn new(dev_path:&str) -> Result<FrameBuffer, ScreenWriterError> {
        let dev = OpenOptions::new().read(true).write(true).open(dev_path)?;

        let vinfo = c::get_var_screeninfo(&dev)?.clone();
        let xres = vinfo.xres;
        let yres = vinfo.yres;
        let bits_per_pixel = vinfo.bits_per_pixel;
        let bytes_per_pixel = bits_per_pixel >> 3;

        c::put_var_screeninfo(&dev, &vinfo)?;

        let finfo = c::get_fix_screeninfo(&dev)?;

        let screen_size = (xres * yres) as usize;

        let screen_buffer_mmap = Mmap::open_with_offset(&dev, Protection::ReadWrite, 0, finfo.smem_len as usize)?;

        let framebuffer = FrameBuffer {
            dev: dev,
            fix_screen_info: finfo,
            screen_buffer: rc::Rc::new(cell::RefCell::new(screen_buffer_mmap)),
            screen_info: ScreenInfo {
                xres : xres,
                yres: yres,
                screen_size : screen_size,
                pixel_def : PixelDef {
                    bits_per_pixel : bits_per_pixel,
                    bytes_per_pixel : bytes_per_pixel,
                    red_offset : vinfo.red.offset,
                    green_offset : vinfo.green.offset,
                    blue_offset : vinfo.blue.offset,
                    transp_offset : vinfo.transp.offset,
                },
                show_debug_info : false,
            },
            var_screen_info: vinfo,
        };

        return Ok(framebuffer)
    }

}



#[derive(Debug)]
pub struct FrameBufferSimulatorPNG<'a> {
    pub screen_info:ScreenInfo,
    pub file_name: &'a str,
}

impl<'a> FrameBufferSimulatorPNG<'a> {

    fn new(file_path:&str, width:u32, height:u32) -> Result<FrameBufferSimulatorPNG, ScreenWriterError> {

        let bits_per_pixel = 32;
        let bytes_per_pixel = 4;
        let page_size = (width * height) as usize;
        let screen_size = page_size;

        let vinfo = c::fb_var_screeninfo {
            xres: width,
            yres: height,
            xres_virtual: width,
            yres_virtual: height,
            xoffset: 0,
            yoffset: 0,
            bits_per_pixel: bits_per_pixel,
            grayscale: 0,
            red: c::fb_bitfield {offset: 16, length: 8, msb_right: 0},
            green: c::fb_bitfield {offset: 8, length: 8, msb_right: 0},
            blue: c::fb_bitfield {offset: 0, length: 8, msb_right: 0},
            transp: c::fb_bitfield {offset: 24, length: 8, msb_right: 0},
            nonstd: 0,
            activate: 0,
            height: 0,
            width: 0,
            accel_flags: 0,
            pixclock: 0,
            left_margin: 0,
            right_margin: 0,
            upper_margin: 0,
            lower_margin: 0,
            hsync_len: 0,
            vsync_len: 0,
            sync: 0,
            vmode: 0,
            rotate: 0,
            colorspace: 0,
            reserved: [0; 4],
        };

        let framebuffer = FrameBufferSimulatorPNG {
            screen_info: ScreenInfo {
                xres : width,
                yres: height,
                screen_size : screen_size,
                pixel_def : PixelDef {
                    bits_per_pixel : bits_per_pixel,
                    bytes_per_pixel : bytes_per_pixel,
                    red_offset : vinfo.red.offset,
                    green_offset : vinfo.green.offset,
                    blue_offset : vinfo.blue.offset,
                    transp_offset : vinfo.transp.offset,
                },
                show_debug_info : false,
            },
            file_name: file_path,
        };

        return Ok(framebuffer)
    }
}

#[derive(Debug)]
pub enum ScreenWriterErrorKind {
    IoctlFailed,
    IoError,
}

#[derive(Debug)]
pub struct ScreenWriterError {
    pub kind: ScreenWriterErrorKind,
    pub details: String,
}

impl ScreenWriterError {
    fn new(kind: ScreenWriterErrorKind, details: &str) -> ScreenWriterError {
        ScreenWriterError { kind: kind, details: String::from(details) }
    }
}

impl error::Error for ScreenWriterError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for ScreenWriterError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description())
    }
}

impl convert::From<io::Error> for ScreenWriterError {
    fn from(err: io::Error) -> ScreenWriterError {
        ScreenWriterError::new(ScreenWriterErrorKind::IoError, err.description())
    }
}