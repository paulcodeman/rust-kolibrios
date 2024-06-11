#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if n == 0 || dest.is_null() || src.is_null() {
        return dest;
    }

    let mut i = 0;
    while i < n {
        *dest.add(i) = *src.add(i);
        i += 1;
    }

    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, x: i32, n: usize) -> *mut u8 {
    if n == 0 || s.is_null() {
        return s;
    }

    let mut i = 0;
    while i < n {
        *s.add(i) = x as u8;
        i += 1;
    }

    s
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *mut u8, s2: *const u8, n: usize) -> i32 {
    if n == 0 || s1.is_null() || s2.is_null() {
        return 0;
    }

    let mut i = 0;
    while i < n {
        let x = *s1.add(i);
        let y = *s2.add(i);
        if x != y {
            return i32::from(x) - i32::from(y);
        }
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn strlen(str: *mut u8) -> usize {
    if str.is_null() {
        return 0;
    }

    let mut len = 0;
    while *str.add(len) != 0 {
        len += 1;
    }

    len
}