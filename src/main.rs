#![no_std]
#![no_main]

use rust_kos::*;
use kos::*;

entry!(main);

fn main() {
    let mut button_id: u16 = 0;
	let num = -12345;
	
    loop {
        match WaitEvent() {
            // Нажата клавиша
            2 => {
                if GetKey() == 27 { // ESC
                    ExitProcess();
                }
            },
            // Нажата кнопка
            3 => {
                button_id = GetButtonID();
                if button_id == 1 { // Выход из программы
                    ExitProcess();
                }
            },
            // Отрисовка окна
            1 => {
                // Значения ширины и высоты окна
                let fw = 800; // замените на реальное значение
                let fh = 750; // замените на реальное значение

                // Значения ширины и высоты элементов управления
                let skin_h = 10; // замените на реальное значение

                // Заголовок окна
                let window_title = "Rainbow (rgb test)\0".as_bytes().as_ptr();

                // Отрисовка окна
                DefineAndDrawWindow(
                    GetScreenWidth() / 2 - fw / 2,
                    GetScreenHeight() / 2 - fh / 2,
                    fw,
                    fh + skin_h,
                    0x33,
                    0xE0DFE3,
                    window_title,
                    0,
                );

                // Надпись на окне
				
                let text = itoa(num).as_ptr();
                WriteText(50, 50, 0x90, 0, text);
            },
            _ => {}
        }
    }
}