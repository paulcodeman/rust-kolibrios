#![no_std]
#![no_main]

#[macro_use]
pub mod kos;


// use crate::dos::allocator::GLOBAL_ALLOCATOR;

#[link_section = ".startup"]
#[no_mangle]
pub extern "C" fn _start() {
    extern "Rust" {
        fn main() -> ();
    }
    unsafe {
        main();
    }
    kos::ExitProcess();
}

#[no_mangle]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) {
    let dest_ptr = dest as *mut u32;
    let src_ptr = src as *const u32;
    let n_u32 = n / 4;
    for i in 0..n_u32 {
        unsafe {
            *dest_ptr.offset(i as isize) = *src_ptr.offset(i as isize);
        }
    }
    let dest_ptr = dest as *mut u8;
    let src_ptr = src as *const u8;
    let n_rem = n % 4;
    for i in 0..n_rem {
        unsafe {
            *dest_ptr.offset((n_u32 * 4 + i) as isize) = *src_ptr.offset((n_u32 * 4 + i) as isize);
        }
    }
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub fn __main() -> () {
            // type check the given path
            let f: fn() -> () = $path;
            f()
        }
    };
}