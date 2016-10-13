// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use time::Duration;
use ops::Sub;
const NSEC_PER_SEC: u64 = 1_000_000_000;

pub struct SteadyTime {
    t: u64
}

impl SteadyTime {
    pub fn now() -> SteadyTime {
        unimplemented!();
    }
}

impl<'a> Sub for &'a SteadyTime {
    type Output = Duration;

    fn sub(self, other: &SteadyTime) -> Duration {
        unimplemented!();
    }
}
