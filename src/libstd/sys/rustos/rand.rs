// Copyright 2013-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rand::Rng;
use mem;
use io;

fn next_u32(mut fill_buf: &mut FnMut(&mut [u8])) -> u32 {
    let mut buf: [u8; 4] = [0; 4];
    fill_buf(&mut buf);
    unsafe { mem::transmute::<[u8; 4], u32>(buf) }
}

fn next_u64(mut fill_buf: &mut FnMut(&mut [u8])) -> u64 {
    let mut buf: [u8; 8] = [0; 8];
    fill_buf(&mut buf);
    unsafe { mem::transmute::<[u8; 8], u64>(buf) }
}

pub struct OsRng {
    // dummy field to ensure that this struct cannot be constructed outside
    // of this module
    _dummy: (),
}

impl OsRng {
    /// Create a new `OsRng`.
    pub fn new() -> io::Result<OsRng> {
        Ok(OsRng { _dummy: () })
    }
}

impl Rng for OsRng {
    fn next_u32(&mut self) -> u32 {
        unimplemented!();
    }
    fn next_u64(&mut self) -> u64 {
        unimplemented!();
    }
    fn fill_bytes(&mut self, v: &mut [u8]) {
        unimplemented!();
    }
}

