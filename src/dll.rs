use crate::sys;
use core::ffi::CStr;

mod console;
pub use console::Console;

#[repr(C)]
struct ImportTableEl {
    pub func_name: *const u8,
    pub func_addr: fn(),
}

pub struct DLL {
    table: *const ImportTableEl,
}

impl DLL {
    pub fn load_dll(name: &CStr) -> Result<DLL, &'static str> {
        unsafe {
            let table = sys::load_dll(name.as_ptr() as *const u8);
            if table.is_null() {
                return Err("Library load error");
            }

            Ok(DLL {
                table: table as *const ImportTableEl,
            })
        }
    }

    pub fn get_func(&self, name: &CStr) -> Result<*const (), &'static str> {
        unsafe {
            let mut i = 0;
            loop {
                let el = self.table.add(i);

                // Ensure we don't dereference a null pointer
                if el.is_null() {
                    return Err("Function not found");
                }

                let cur_name = CStr::from_ptr((*el).func_name as *const i8);
                if cur_name == name {
                    return Ok((*el).func_addr as *const ());
                }

                i += 1;
            }
        }
    }
}