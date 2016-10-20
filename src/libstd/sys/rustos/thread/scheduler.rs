// TODO(ryan): it really looks like bulk of libgreen could be used here where pthread <-> core

use core::prelude::*;
use core::cell::UnsafeCell;
use core::mem::{transmute, transmute_copy};
use core::ptr;
use io;
use time::Duration;

use alloc::boxed::{Box, FnBox};

use super::linked_list::{LinkedList, Node};

use super::context::Context;
use super::stack::Stack;

use super::super::arch::cpu;

use fringe;
use super::fringe_wrapper;

lazy_static! {
  static ref SCHEDULER: UnsafeCell<Scheduler> = UnsafeCell::new(Scheduler::new());
}

pub fn get_scheduler() -> &'static mut Scheduler {
    unsafe { transmute(SCHEDULER.get()) }
}

// thread control block
struct Tcb {
  group: fringe_wrapper::Group<'static, (), ThreadRequest, fringe::OwnedStack>,
}

unsafe impl Send for Tcb {}

type F = &'static (Fn(Box<Node<Tcb>>) + Sync);

// Request of thread to scheduler
enum ThreadRequest {    
    Yield,
    Unschedule(F),
    Schedule(Tcb),
}

pub struct Scheduler {
  queue: LinkedList<Tcb>,
}

impl Scheduler {
  
  fn new() -> Scheduler {
    let mut s = Scheduler { queue: LinkedList::new()  }; 
    s
  }
  
  fn request(&mut self, request: ThreadRequest) {
    debug!("suspending");
    unsafe { self.queue.front().unwrap().group.suspend(request); }
  }
  
  pub fn bootstrap_start<F>(f: F) -> ! where F: FnOnce() + Send + 'static {
    get_scheduler().run(Self::new_tcb(f))
  }
  
  fn run(&mut self, start: Tcb) -> ! {
    // scheduler now takes control of the CPU
    self.queue.push_front(Self::new_tcb(Self::idle));
    self.queue.push_front(start);
    
    loop {
        let request = unsafe { self.queue.front_mut().unwrap().group.resume(()) };
        debug!("got request");
        match request {
            Some(req) => match req {
                ThreadRequest::Yield => {
                    debug!("Requesting yield");
                    let current = self.queue.pop_front().unwrap();
                    self.queue.push_back(current);
                },
                ThreadRequest::Unschedule(func) => {
                    debug!("Requesting Unschedule");
                    let node: Box<Node<Tcb>> = self.queue.pop_front_node().unwrap();
                    func(node);
                },
                ThreadRequest::Schedule(tcb) => {
                    debug!("Requesting schedule");
                    self.queue.push_back(tcb);
                },
            },
            None => {self.queue.pop_front();},
        }
        
    }
  }
  
  fn idle() {
    get_scheduler().request(ThreadRequest::Yield);
    loop {
        // TODO should idle and yield in here...
    }
  }
  
  fn new_tcb<F>(func: F) -> Tcb where F: FnOnce() + Send + 'static {
    const STACK_SIZE: usize = 1024 * 1024;
    let stack = fringe::OwnedStack::new(STACK_SIZE);
  
    unsafe {
        Tcb { group: fringe_wrapper::Group::new(func, stack) }
    }
    
  }
  
  
}

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

pub struct Mutex {
    taken: UnsafeCell<bool>,
    sleepers: UnsafeCell<LinkedList<Tcb<>>>
}

impl Mutex {

    pub const fn new() -> Mutex {
        Mutex { taken: UnsafeCell::new(false), sleepers: UnsafeCell::new(new_linked_list!())}
    }
    
    pub unsafe fn init(&mut self) {}

    pub unsafe fn lock(&self) {
        cpu::current_cpu().disable_interrupts();
        while *self.taken.get() {
            //get_scheduler().request(ThreadRequest::Unschedule(&|me: Box<Node<Tcb>>| {
            //    (*self.sleepers.get()).push_back_node(me);
            //}));
        }
        *self.taken.get() = true;
        cpu::current_cpu().enable_interrupts();
    }
    
    pub unsafe fn  try_lock(&self) -> bool {
        let mut ret;
        cpu::current_cpu().disable_interrupts();
        if *self.taken.get() {
            ret = false
        } else {
            *self.taken.get() = true;
            ret = true;
        }
        cpu::current_cpu().enable_interrupts();
        return ret;
    }
    
    pub unsafe fn unlock(&self) {
        cpu::current_cpu().disable_interrupts();
        assert!(*self.taken.get());
        *self.taken.get() = false;
        //match (*self.sleepers.get()).pop_front() {
        //    Some(tcb) => get_scheduler().request(ThreadRequest::Schedule(tcb)),
        //    None => ()
        //}
        cpu::current_cpu().enable_interrupts();
    }
    
    pub unsafe fn destroy(&self) {
    }

}

unsafe impl Send for Condvar {}
unsafe impl Sync for Condvar {}

pub struct Condvar {
    sleepers: UnsafeCell<LinkedList<Tcb>>
}

impl Condvar {

    pub const fn new() -> Condvar {
        unsafe { Condvar { sleepers: UnsafeCell::new(new_linked_list!()) } }
    }
    
    pub unsafe fn init(&mut self) {}

    pub unsafe fn notify_one(&self) {
        cpu::current_cpu().disable_interrupts();
        //match (*self.sleepers.get()).pop_front() {
        //    Some(tcb) => get_scheduler().request(ThreadRequest::Schedule(tcb)),
        //    None => ()
        //}
        cpu::current_cpu().enable_interrupts();
    }

    pub unsafe fn notify_all(&self) {
        cpu::current_cpu().disable_interrupts();
        while !(*self.sleepers.get()).is_empty() {
            self.notify_one();
        }
        cpu::current_cpu().enable_interrupts();
    }

    pub unsafe fn wait(&self, mutex: &Mutex) {
        cpu::current_cpu().disable_interrupts();
        mutex.unlock();
        //get_scheduler().get_scheduler().request(ThreadRequest::Unschedule(&|me: Box<Node<Tcb>>| { 
        //    (*self.sleepers.get()).push_back_node(me);
        //}));
        mutex.lock();
        cpu::current_cpu().enable_interrupts();
    }
    
    pub unsafe fn wait_timeout(&self, mutex: &Mutex, dur: Duration) -> bool {
        unimplemented!();
    }

    pub unsafe fn destroy(&self) {
    }

}

unsafe impl Send for RWLock {}
unsafe impl Sync for RWLock {}

pub const RWLOCK_INIT: RWLock = RWLock;

pub struct RWLock;

impl RWLock {

    pub const fn new() -> RWLock { RWLock }

    pub unsafe fn read(&self) { unimplemented!(); }

    pub unsafe fn try_read(&self) -> bool { unimplemented!(); }

    pub unsafe fn write(&self) { unimplemented!(); }

    pub unsafe fn try_write(&self) -> bool { unimplemented!(); }

    pub unsafe fn read_unlock(&self) { unimplemented!(); }

    pub unsafe fn write_unlock(&self) { unimplemented!(); }

    pub unsafe fn destroy(&self) { unimplemented!(); }

}

unsafe impl Send for ReentrantMutex {}
unsafe impl Sync for ReentrantMutex {}

pub struct ReentrantMutex;

impl ReentrantMutex {
    pub unsafe fn uninitialized() -> ReentrantMutex {
        unimplemented!();
    }

    pub unsafe fn init(&mut self) {
        unimplemented!();
    }

    pub unsafe fn lock(&self) {
        unimplemented!();
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        unimplemented!();
    }

    pub unsafe fn unlock(&self) {
        unimplemented!();
    }

    pub unsafe fn destroy(&self) {
        unimplemented!();
    }
}

pub struct Thread;

unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

impl Thread {
    pub unsafe fn new<'a>(stack: usize, p: Box<FnBox() + 'a>)
                          -> io::Result<Thread> {
        unimplemented!();
    }

    pub fn yield_now() {
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub fn set_name(name: &str) {
    }

    pub fn sleep(dur: Duration) {
    }

    pub fn join(self) {
    }
}

pub type Key = usize;

#[inline]
pub unsafe fn create(dtor: Option<unsafe extern fn(*mut u8)>) -> Key {
    unimplemented!();
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    unimplemented!();
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    unimplemented!();
}

#[inline]
pub unsafe fn destroy(key: Key) {
    unimplemented!();
}


fn inner_thread_test(arg: usize) {
  debug!("arg is {}", arg)
}

extern "C" fn test_thread() {
  debug!("in a test thread!");
  inner_thread_test(11);
  unsafe {
    let s = get_scheduler();
    debug!("leaving test thread!");
    s.request(ThreadRequest::Yield);
  }
}

pub fn thread_stuff() {
  debug!("starting thread test");
  unsafe {
    let s: &mut Scheduler = get_scheduler();

    debug!("orig sched 0x{:x}", transmute_copy::<_, u32>(&s));
    //loop {};
    let t = || { test_thread() };
    s.request(ThreadRequest::Schedule(Scheduler::new_tcb(t)));
    debug!("schedule okay");
    s.request(ThreadRequest::Yield);
    debug!("back");
  }
}
