use crate::dll::DLL;
use core::ffi::CStr;
use core::mem::transmute;

extern "C" {
    fn strlen(s: *const u8) -> usize;
}

static mut BUFFER: [u8; 1000] = [0u8; 1000];

pub struct Console {
    con_init: extern "stdcall" fn(u32, u32, u32, u32, *const u8),
    con_gets: extern "stdcall" fn(*const u8, u32),
    con_write_string: extern "stdcall" fn(*const u8, u32),
    con_exit: extern "stdcall" fn(bool),
}

impl Console {
    pub fn import(path: Option<&CStr>) -> Result<Self, &str> {
        let lib = DLL::load_dll(path.unwrap_or(c"/sys/lib/console.obj"));
        match lib {
            Err(e) => return Err(e),
            Ok(dll) => unsafe {
                Ok(Console {
                    con_init: transmute(dll.get_func(c"con_init").ok().unwrap()),
					con_gets: transmute(dll.get_func(c"con_gets").ok().unwrap()),
                    con_write_string: transmute(dll.get_func(c"con_write_string").ok().unwrap()),
                    con_exit: transmute(dll.get_func(c"con_exit").ok().unwrap()),
                })
            },
        }
    }

    pub fn init(&self, x: u32, y: u32, width: u32, height: u32, title: &CStr) {
        (self.con_init)(x, y, width, height, title.as_ptr() as *const u8);
    }

    pub fn gets(&self) -> &str {
        unsafe {
            (self.con_gets)(BUFFER.as_mut_ptr(), BUFFER.len() as u32);
            let length = strlen(BUFFER.as_ptr());
            core::str::from_utf8(&BUFFER[..length]).unwrap_or("")
        }
    }

    pub fn write_string(&self, text: &str) {
        (self.con_write_string)(text.as_ptr(), text.len() as u32);
    }

    pub fn exit(self, close_window: bool) {
        (self.con_exit)(close_window);
    }

    pub fn write_char(&self, c: char) {
        let mut buf = [0u8; 4];
        let s = c.encode_utf8(&mut buf);
        self.write_string(s);
    }
}
