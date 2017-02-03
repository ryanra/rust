use ::sys::thread::{Mutex, Condvar};
use ::alloc::arc::Arc;

pub struct InterruptHandler {
  mutex: Mutex<()>,
  condvar: Condvar,
}

impl InterruptHandler {

  pub fn new() -> InterruptHandler {
    InterruptHandler { mutex: Mutex::new(()), condvar: Condvar::new() }
  }

  pub fn notify(&self) {
    self.condvar.notify_all();
  }

  pub fn wait(&self) {
    self.condvar.wait(self.mutex.lock().unwrap());
  }

}

impl Iterator for InterruptHandler {
  type Item = ();

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.wait())
  }

}
