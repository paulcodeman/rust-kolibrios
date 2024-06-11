use crate::sys;
use core::ffi::CStr;

#[derive(Clone, Copy)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    pub fn r(&self) -> u8 {
        self.0
    }

    pub fn g(&self) -> u8 {
        self.1
    }

    pub fn b(&self) -> u8 {
        self.2
    }

    pub fn as_rgb_val(self) -> u32 {
        (self.0 as u32) << 16 | (self.1 as u32) << 8 | (self.2 as u32)
    }
}

pub struct Dot {
    pub x: u32,
    pub y: u32,
}

pub struct Size {
    pub width: u32,
    pub height: u32,
}

pub fn display_message<'a>(start: Dot, color: Color, text: &'a CStr, bg_color: Option<Color>) {
    // XX=ABFFCSSS
    const UTF8_FLAG: u32 = (3 << 4) << 24; // FF
    const BG_FLAG: u32 = (1 << 6) << 24; // B
    const ASCIIZ_FLAG: u32 = (1 << 7) << 24; // A

    unsafe {
        sys::display_message(
            start.x << 16 | start.y,
            color.as_rgb_val() | BG_FLAG * bg_color.is_some() as u32 | UTF8_FLAG | ASCIIZ_FLAG,
            text.as_ptr() as u32,
            0,
            bg_color.unwrap_or(Color(0, 0, 0)).as_rgb_val(),
        );
    }
}

pub fn display_message_str<'a>(start: Dot, color: Color, text: &'a str, bg_color: Option<Color>) {
    // XX=ABFFCSSS
    const UTF8_FLAG: u32 = (3 << 4) << 24; // FF
    const BG_FLAG: u32 = (1 << 6) << 24; // B

    unsafe {
        sys::display_message(
            start.x << 16 | start.y,
            color.as_rgb_val() | BG_FLAG * bg_color.is_some() as u32 | UTF8_FLAG,
            text.as_ptr() as u32,
            text.len() as u32,
            bg_color.unwrap_or(Color(0, 0, 0)).as_rgb_val(),
        );
    }
}
