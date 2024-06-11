use crate::sys::{self, set_event_mask};

pub fn exit() -> ! {
    unsafe { sys::exit() }
}

#[non_exhaustive]
pub enum Event {
    Redraw = 1 << 0,
    KeyPress = 1 << 1,
    BtnPress = 1 << 2,
    BgRedraw = 1 << 4,
    Mouse = 1 << 5,
    IPC = 1 << 6,
    Network = 1 << 7,
    Debug = 1 << 8,
}

pub fn fetch_event(flags: u32) -> Option<Event> {
    let old_mask = unsafe { sys::set_event_mask(flags as u32) };
    let e = match unsafe { sys::wait_event() } {
        1 => Some(Event::Redraw),
        2 => Some(Event::KeyPress),
        3 => Some(Event::BtnPress),
        5 => Some(Event::BgRedraw),
        6 => Some(Event::Mouse),
        7 => Some(Event::IPC),
        8 => Some(Event::Network),
        9 => Some(Event::Debug),
        _ => None,
    };
    unsafe { set_event_mask(old_mask) };
    e
}
