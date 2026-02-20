use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

/// A simplified version of Vec<T>
pub struct MiniVec<T> {
    ptr: *mut T,       // pointer to heap memory
    len: usize,        // number of initialized elements
    capacity: usize,   // total allocated capacity
}

impl<T> MiniVec<T> {

    /// Create an empty MiniVec
    pub fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),  // no allocation yet
            len: 0,
            capacity: 0,
        }
    }

    /// Add an element to the end
    pub fn push(&mut self, value: T) {

        // If we are out of space, allocate more
        if self.len == self.capacity {
            self.grow();
        }

        unsafe {
            // Write value into uninitialized memory
            self.ptr.add(self.len).write(value);
        }

        self.len += 1;
    }

    /// Increase allocation capacity
    fn grow(&mut self) {

        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };

        // Calculate memory layout for array of T
        let layout = Layout::array::<T>(new_capacity).unwrap();

        // Allocate new memory
        let new_ptr = unsafe { alloc(layout) as *mut T };

        if !self.ptr.is_null() {
            unsafe {
                // Copy existing elements into new memory
                ptr::copy_nonoverlapping(
                    self.ptr,
                    new_ptr,
                    self.len,
                );

                // Deallocate old memory
                let old_layout =
                    Layout::array::<T>(self.capacity).unwrap();

                dealloc(self.ptr as *mut u8, old_layout);
            }
        }

        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }
}

/// Custom destructor
impl<T> Drop for MiniVec<T> {
    fn drop(&mut self) {
        if self.capacity != 0 {
            unsafe {

                // Drop each initialized element
                for i in 0..self.len {
                    ptr::drop_in_place(self.ptr.add(i));
                }

                // Free heap memory
                let layout =
                    Layout::array::<T>(self.capacity).unwrap();

                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

fn main() {

    let mut v = MiniVec::new();

    v.push(10);
    v.push(20);
    v.push(30);

    println!("MiniVec length: {}", v.len);
}
