// stack interface for RustOS

use core::prelude::*;
use vec::Vec;

pub struct Stack {
    v: Vec<u8>
}

impl Stack {

    pub fn new(size: usize) -> Stack {
        Stack { v: Vec::with_capacity(size) } 
    }
    
    /// Point to the low end of the allocated stack
    pub fn start(&self) -> *const usize {
        self.v.as_ptr() as *const usize
    }

    /// Point one usize beyond the high end of the allocated stack
    pub fn end(&self) -> *const usize {
        unsafe { self.v.as_ptr().offset(self.v.capacity() as isize) as *const usize } // TODO(ryan) overflow on cast?
    }

}
