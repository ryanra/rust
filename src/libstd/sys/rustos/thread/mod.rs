extern crate barn;

use self::barn::{scheduler, basic, lock};
use ::fringe;

static mut SCHEDULER: Option<Scheduler> = None;

pub type Scheduler = scheduler::Scheduler<basic::Unit>;
pub type Mutex<T> = lock::Mutex<T, basic::Unit>;
pub type MutexGuard<'a, T> = lock::MutexGuard<'a, T, basic::Unit>;
pub type Condvar = lock::Condvar<basic::Unit>;
pub type RwLock<T> =  lock::RwLock<T, basic::Unit>;
pub type RwLockReadGuard<'a, T> = lock::RwLockReadGuard<'a, T, basic::Unit>;
pub type RwLockWriteGuard<'a, T> = lock::RwLockWriteGuard<'a, T, basic::Unit>;
pub type Thread = scheduler::Thread<basic::Unit>;

pub fn start<F: FnOnce() + Send + 'static>(f: F) {
  let stack = fringe::OwnedStack::new(10 * 1024);
  let thread = scheduler::Thread::new(stack, f);

  let mut q = basic::Queue::new();
  q.push_front(thread);

  unsafe {
    SCHEDULER = Some(Scheduler::new(q));
    SCHEDULER.as_mut().unwrap().run();
  }
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
