use crate::sys;
use crate::throw_new;
use alloc::string::String;
use core::ffi::CStr;

// Сделаем трейт публичным
pub trait Debuggable {
    fn data_iter(self) -> impl Iterator<Item = u8>;
}

impl Debuggable for &str {
    fn data_iter(self) -> impl Iterator<Item = u8> {
        self.bytes()
    }
}

impl Debuggable for &String {
    fn data_iter(self) -> impl Iterator<Item = u8> {
        self.as_bytes().iter().copied()
    }
}

impl Debuggable for &CStr {
    fn data_iter(self) -> impl Iterator<Item = u8> {
        self.to_bytes().iter().copied()
    }
}

pub enum Lang {
    English,
    Finnish,
    German,
    Russian,
    French,
    Estonian,
    Spanish,
    Italian,
}

pub fn get_lang() -> Lang {
    unsafe {
        let l = sys::get_lang();
        return match l {
            1 => Lang::English,
            2 => Lang::Finnish,
            3 => Lang::German,
            4 => Lang::Russian,
            5 => Lang::French,
            6 => Lang::Estonian,
            7 => Lang::Spanish,
            8 => Lang::Italian,
            _ => {
                throw_new!(format!("Unknown lang: {}", l));
                Lang::English
            }
        };
    }
}

pub fn debug_write<Str: Debuggable>(text: Str) {
    for byte in text.data_iter() {
        unsafe {
            sys::_debug_write(byte);
        }
    }
}