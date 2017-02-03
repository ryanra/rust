extern crate barn;

use ::fringe;

pub use self::barn::basic::*;
use self::barn::scheduler::{self, Request};

static mut SCHEDULER: Option<Scheduler> = None;

pub fn start<F: FnOnce() + Send + 'static>(f: F) {
  let stack = fringe::OwnedStack::new(16 * 1024);
  let thread = Thread::new(stack, f);

  let mut q = Queue::new();
  q.push_front(thread);

  unsafe {
    SCHEDULER = Some(Scheduler::new(q));
    debug!("scheduler is none? {}", SCHEDULER.is_none());
    //loop {}
    SCHEDULER.as_mut().unwrap().run();
  }
}

pub fn spawn<F: FnOnce() + Send + 'static>(f: F, size: usize) {
  let t = Thread::new(fringe::OwnedStack::new(size), f);
  Thread::suspend(Request::Schedule(scheduler::Node::<Unit>::new(t)));
}

pub fn yield_now() {
  Thread::suspend(Request::Yield);
}

/// A type indicating whether a timed wait on a condition variable returned
/// due to a time out or not.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[stable(feature = "wait_timeout", since = "1.5.0")]
pub struct WaitTimeoutResult(bool);

impl WaitTimeoutResult {
    /// Returns whether the wait was known to have timed out.
    #[stable(feature = "wait_timeout", since = "1.5.0")]
    pub fn timed_out(&self) -> bool {
        self.0
    }
}
