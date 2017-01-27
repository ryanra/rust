// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sync::{Arc, Mutex};

pub struct Lazy<T> {
    data: Arc<Mutex<Option<Arc<T>>>>,
    init: fn() -> Arc<T>,
}

unsafe impl<T> Sync for Lazy<T> {}

impl<T: Send + Sync + 'static> Lazy<T> {
    pub const fn new(init: fn() -> Arc<T>) -> Lazy<T> {
        Lazy {
            data: Arc::new(Mutex::new(None)),
            init: init
        }
    }

    pub fn get(&'static self) -> Option<Arc<T>> {
      let mut maybe_data = self.data.lock().unwrap();
      let result: Arc<T> = match maybe_data {
        None => {
          let res = (self.init)();
          *maybe_data = res;
          res
        },
        Some(res) => res,
      };
      Some(result)
    }

}
