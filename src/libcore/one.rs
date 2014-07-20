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