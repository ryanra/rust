// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;
use core::prelude::*;

pub struct Handler;

impl Handler {
    pub unsafe fn new() -> Handler {
        unimplemented!();
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        unimplemented!();
    }
}

pub unsafe fn init() {
}

pub unsafe fn cleanup() {
}

pub unsafe fn make_handler() -> Handler {
    unimplemented!();
}

pub unsafe fn drop_handler(_handler: &mut Handler) {
}