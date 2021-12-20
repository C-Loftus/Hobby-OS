use core::char::ParseCharError;
use core::fmt::{Result, Error};
use core::num::IntErrorKind;
use core::result;
use vga::drawing;
use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter, Graphics320x200x256};

use crate::vga_buffer::Color;

pub fn hello_world_graphics() -> () {


    let mode = Graphics640x480x16::new();
    mode.set_mode();
    mode.clear_screen(Color16::Black);
    mode.draw_line((80, 60), (80, 420), Color16::White);
    mode.draw_line((80, 60), (540, 60), Color16::White);
    mode.draw_line((80, 420), (540, 420), Color16::White);
    mode.draw_line((540, 420), (540, 60), Color16::White);
    mode.draw_line((80, 90), (540, 90), Color16::White);
    for (offset, character) in "Hello World!".chars().enumerate() {
        mode.draw_character(270 + offset * 8, 72, character, Color16::White)
    }

}
const HEADER_LEN: usize = 8+4+4;
const MAGIC_VALUE: &[u8; 8] = b"farbfeld";
pub struct farbfeld<'a> {
    pub width: usize,
    pub height: usize,
    pub pixels: &'a[u8; 37136]
}
impl farbfeld<'_> {
    pub fn new () -> Option<farbfeld<'static>> {

        let p = include_bytes!("../tests/h.ff");

        let head = &p;
        if &head[..8] != b"farbfeld" {
             return None;
        }

        Some (
            farbfeld {
            width : as_u32_be(&head[8..12]) as usize,
            height: as_u32_be(&head[12..16]) as usize, 
            pixels: p
            }
        )
    }
    pub fn display(&self) -> () {
        use core::convert::TryInto;
        let mode = Graphics320x200x256::new();
        mode.set_mode();
        mode.clear_screen(0x9);
        for h in 0..self.height {
             for w in 0..self.width {
                let start = h.try_into().unwrap();
                let end = w.try_into().unwrap();
                mode.set_pixel(start, end, self.pixels[HEADER_LEN + h + w]);
            }   
        }    
    }
}

fn as_u32_be(array: &[u8]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}

fn as_u32_le(array: &[u8]) -> u32 {
    ((array[0] as u32) <<  0) +
    ((array[1] as u32) <<  8) +
    ((array[2] as u32) << 16) +
    ((array[3] as u32) << 24)
}

fn convert(input: u8) -> Color16 {
    return match input {
        0x1   => Color16::Blue        ,
        0x2   => Color16::Green,
        0x3   => Color16::Cyan,
        0x4  =>  Color16::Red,
        0x5   => Color16::Magenta,
        0x6   => Color16::Brown,
        0x7  =>  Color16::LightGrey,
        0x8   => Color16::DarkGrey,
        0x9   => Color16::LightBlue,
        0xA   => Color16::LightGreen,
        0xB   => Color16::LightCyan,
        0xC   => Color16::LightRed,
        0xD   => Color16::Pink,
        0xE  =>  Color16::Yellow,
        0xF   => Color16::White,
        _   => Color16::Black,
    }
}

