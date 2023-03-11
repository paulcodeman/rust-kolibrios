#![feature(asm)]
use core::arch::asm;

const EV_REDRAW: u32 = 1;
const EV_KEY: u32 = 2;
const EV_BUTTON: u32 = 3;
const EV_DESKTOP: u32 = 5;
const EV_MOUSE: u32 = 6;
const EV_IPC: u32 = 7;
const EV_NETWORK: u32 = 8;
const EV_DEBUG: u32 = 9;

// pub mod allocator;
pub mod panic;

pub fn WaitEvent() -> u32 {
    let eax: u32;
    unsafe {
        asm!(
            "mov eax, 10",
            "int 0x40",
            out("eax") eax,
        );
    }
    eax
}

pub fn GetButtonID() -> u16 {
    let eax: u32;
    let result: u16;
    unsafe {
        asm!(
            "mov eax, 17",
            "int 0x40",
            "shr eax, 8",
            out("eax") eax,
        );
    }
    result = eax as u16;
    result
}

pub extern "C" fn sleep(s: u32) {
	unsafe {
		asm!(
			"mov eax, 5",
			"int 0x40",
			in("ebx") s
		);
    }
}

pub fn ExitProcess() -> ! {
    let mut eax = -1i32 as u32;
    unsafe {
        asm!(
            "int 0x40",
            inout("eax") eax,
        );
    }
    // этот код никогда не будет достигнут,
    // поскольку процесс будет завершен
    unreachable!()
}


pub fn DefineAndDrawWindow(x: i32, y: i32, w: i32, h: i32, window_type: u32, bgcolor: u32, title: *const u8, flags: u32) {
    let eax: u32 = 0;
    let ebx: i32 = (x << 16) + w;
    let ecx: i32 = (y << 16) + h;
    let edx: u32 = (window_type << 24) | bgcolor;
    let edi: *const u8 = title;

	unsafe {
		asm!("pusha");
        asm!(
            "int 0x40
			mov esi, 0",
            in("eax") eax,
			in("ebx") ebx,
			in("ecx") ecx,
			in("edx") edx,
			in("edi") edi,
        );
		asm!("popa");
    }
}

pub fn GetKey() -> u32 {
	let eax: u32;
    unsafe {
        asm!(
            "mov eax, 2",
            "int 0x40",
			out("eax") eax,
        );
    }
	eax
}

pub fn SetKeyboardMode(ecx: u32) {
    unsafe {
        asm!(
            "mov eax, 66",
            "mov ebx, 1",
            "int 0x40",
            in("ecx") ecx,
        );
    }
}

pub fn GetKeyModifier() -> u32 {
    let eax: u32;
    unsafe {
        asm!(
            "mov eax, 66",
            "mov ebx, 3",
            "int 0x40",
            out("eax") eax,
        );
    }
    eax
}

pub fn WriteText(x: i32, y: i32, font_type: u32, color: u32, str_offset: *const u8) {
    let eax: u32 = 4;
    let ebx: i32 = (x << 16) | y;
    let ecx: u32 = ((font_type as u32) << 24) | color;
    let edx: *const u8 = str_offset;
    unsafe {
		asm!("pusha");
        asm!(
            "int 0x40",
            in("eax") eax,
            in("ebx") ebx,
            in("ecx") ecx,
            in("edx") edx,
        );
		asm!("popa");
    }
}


pub fn GetScreenWidth() -> i32 {
    let eax: i32;
    unsafe {
        asm!("mov eax, 14
              int 0x40
              shr eax, 16",
              out("eax") eax,
        );
    }
    eax
}


pub fn GetScreenHeight() -> i32 {
    let eax: i32;
    unsafe {
        asm!(
            "mov eax, 14",
            "int 0x40",
            "and eax, 0x0000FFFF",
            out("eax") eax,
        );
    }
    eax
}

pub fn PutImage(x: u32, y: u32, w: u32, h: u32, data_offset: u32) {
    let eax: u32 = 7;
    let ebx: u32 = data_offset;
    let ecx: u32 = (w << 16) | h;
    let edx: u32 = (x << 16) | y;
	unsafe {
		asm!("pusha");
        asm!(
            "int 0x40",
            in("eax") eax,
            in("ebx") ebx,
            in("ecx") ecx,
            in("edx") edx,
        );
		asm!("popa");
    }
}

pub fn itoa(num: i32) -> [u8; 11] {
    let mut num = num;
    let mut result = [b'0'; 11];
    let mut i = 10;

    if num < 0 {
        result[0] = b'-';
        num = -num;
    }

    while num > 0 {
        result[i] = b'0' + (num % 10) as u8;
        num /= 10;
        i -= 1;
    }

    result
}

pub fn malloc(size: usize) -> *mut u8 {
    let result: *mut u8;
    unsafe {
        asm!("pusha");
        asm!(
            "int 0x40",
            in("eax") 68,
            in("ebx") 12,
            in("ecx") size,
            lateout("eax") result
        );
        asm!("popa");
    }
    result
}

pub fn realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    let result: *mut u8;
    unsafe {
        asm!("pusha");
        asm!(
            "int 0x40",
            in("eax") 68,
            in("ebx") 12,
            in("ecx") size,
            in("edx") ptr,
            lateout("eax") result
        );
        asm!("popa");
    }
    result
}

pub fn free(ptr: *mut u8) {
    unsafe {
        asm!("pusha");
        asm!(
            "int 0x40",
            in("eax") 68,
            in("ebx") 13,
            in("ecx") ptr
        );
        asm!("popa");
    }
}

