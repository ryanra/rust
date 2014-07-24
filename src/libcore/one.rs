// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Stop-gap


/// It's a struct :O
pub struct Once {
    is_called: bool
}

/// Initialization value for static `Once` values.
pub static ONCE_INIT: Once = Once { is_called: false };

impl Once {
    
    /// Perform an initialization routine once and only once.
    /// TODO(ryan): since I don't have any kind of mutex's yet,
    /// this isn't really safe
    #[inline(always)]
    pub fn doit(&mut self, f: ||) {
	if self.is_called {
	  return;
	} else {
	  self.is_called = true;
	  f();
	}
    }
}