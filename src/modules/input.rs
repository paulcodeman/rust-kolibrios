use crate::sys;

pub fn fetch_key() -> Option<u8> {
    let res = unsafe { sys::pressed_key() };
    if res == 1 {
        None
    } else {
        Some(((res >> 8) & 0xff) as u8)
    }
}
