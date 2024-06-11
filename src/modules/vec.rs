use core::ptr::{self, null_mut};
use core::alloc::Layout;
use crate::sys::{alloc, free};
extern crate alloc;

const INITIAL_CAPACITY: usize = 4;

pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Self {
            ptr: null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }

        unsafe {
            let end = self.ptr.add(self.len);
            ptr::write(end, value);
            self.len += 1;
        }
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            INITIAL_CAPACITY
        } else {
            self.capacity * 2
        };

        let new_layout = Layout::array::<T>(new_capacity).expect("Layout creation failed");

        unsafe {
            let new_ptr = alloc(new_layout.size()) as *mut T;

            if new_ptr.is_null() {
                alloc::alloc::handle_alloc_error(new_layout);
            }

            if !self.ptr.is_null() {
                ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
                free(self.ptr as *mut u8);
            }

            self.ptr = new_ptr;
            self.capacity = new_capacity;
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                for i in 0..self.len {
                    let element_ptr = self.ptr.add(i);
                    ptr::drop_in_place(element_ptr);
                }
                free(self.ptr as *mut u8);
            }
        }
    }
}