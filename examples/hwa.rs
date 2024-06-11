#![no_std]
#![no_main]

use alloc::ffi::CString;
use core::ffi::CStr;
use kos::{
    graphics::{display_message, Color, Dot, Size},
    input::fetch_key,
    system::{get_lang, Lang},
    threads::{exit, fetch_event, Event},
    windows::{
        define_button, define_window, end_window_draw, get_button_id, start_window_draw,
        WindowKind, WindowParams, CLOSE_BUTTON,
    },
};

const HEADER: &CStr = c"Hey Kolibri";
const MSG: &CStr = c"Hello from Rust!";
const BTN: u32 = 42;

#[macro_use]
extern crate alloc;

fn draw_window(c: usize) {
    start_window_draw();

    define_window(
        Dot { x: 50, y: 50 },
        Size {
            width: 300,
            height: 400,
        },
        WindowParams {
            color: Color::rgb(0xff, 0xff, 0xff),
            kind: WindowKind::Themed,
            title: Some(HEADER),
        },
    );

    display_message(
        Dot { x: 10, y: 10 },
        Color::rgb(0x66, 0x22, 0x22),
        MSG,
        None,
    );

    let btn_str = match get_lang() {
        Lang::German => format!("Taste gedrückt: {} mal", c),
        Lang::Russian => format!("Кнопка нажата: {} раз", c),
        Lang::French => format!("Button enfoncé : {} fois", c),
        _ => format!("Button pressed: {} times", c),
    };

    display_message(
        Dot { x: 10, y: 30 },
        Color::rgb(0, 0, 0),
        CString::new(btn_str.as_bytes())
            .expect("CString error")
            .as_c_str(),
        None,
    );

    define_button(
        Dot { x: 10, y: 70 },
        Size {
            width: 70,
            height: 15,
        },
        BTN,
        true,
        true,
        Some(Color::rgb(128, 255, 128)),
    );

    end_window_draw();

    return;
}

fn button_handler(c: &mut usize) {
    let btn_id = get_button_id();

    if btn_id.is_some() {
        match btn_id.unwrap() {
            CLOSE_BUTTON => exit(),
            BTN => {
                *c += 1;
                draw_window(*c);
            }
            _ => {}
        }
    }
}

#[no_mangle]
fn kol_main() {
    let mut c = 0;

    while let Some(ev) =
        fetch_event((Event::Redraw as u32) | (Event::KeyPress as u32) | (Event::BtnPress as u32))
    {
        match ev {
            Event::Redraw => draw_window(c),
            Event::KeyPress => drop(fetch_key()),
            Event::BtnPress => button_handler(&mut c),
            _ => break,
        }
    }
}
