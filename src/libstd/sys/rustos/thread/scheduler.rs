// TODO(ryan): it really looks like bulk of libgreen could be used here where pthread <-> core

use core::prelude::*;
use core::cell::UnsafeCell;
use core::mem::{transmute, transmute_copy};
use core::ptr;
use io;
use time::Duration;

use alloc::boxed::{Box, FnBox};

use super::linked_list::LinkedList;

use super::context::Context;
use super::stack::Stack;

use super::super::arch::cpu;

// thread control block
struct Tcb { 
  context: Context,
}

// invariant: current thread is at front of queue
pub struct Scheduler {
  queue: LinkedList<Tcb>
}

lazy_static! {
  static ref SCHEDULER: UnsafeCell<Scheduler> = UnsafeCell::new(Scheduler::new());
}

pub fn get_scheduler() -> &'static mut Scheduler {
    unsafe { transmute(SCHEDULER.get()) }
}

#[no_mangle]
extern "C" fn run_thunk(thunk: &Fn() -> ()) {
  debug!("in run_thunk");
  thunk();
  unreachable!("didn't unschedule finished thread");
}

impl Scheduler {
  
  pub fn new() -> Scheduler {
    let idle_task = || {
        loop {
            trace!("in idle task 1");
            trace!("wait done");
            get_scheduler().switch();
            trace!("switch done");
            loop {}
        }
    };

    let mut s = Scheduler { queue: LinkedList::new() }; 
    let tcb = s.new_tcb(box idle_task);
    s.queue.push_front(tcb);
    s
  }
  
  pub fn bootstrap_start(&mut self) -> ! {
    // scheduler now takes control of the CPU
    // current context is discarded and front of queue is started
    let mut dont_care = Context::empty();
    Context::swap(&mut dont_care, &self.queue.front_mut().unwrap().context);
    unreachable!();
  }
  
  fn new_tcb(&self, func: Box<Fn() -> ()>) -> Tcb {
    const STACK_SIZE: usize = 1024 * 1024;
    let stack = Stack::new(STACK_SIZE);

    let p = move || {
      unsafe { cpu::current_cpu().enable_interrupts(); }
      func();
      get_scheduler().unschedule_current();
    };
    
    let c = Context::new(run_thunk, box p as Box<Fn() -> ()>, stack);
    Tcb { context: c }
  }
  
  pub fn schedule(&mut self, func: Box<Fn() -> ()>) {
    let new = self.new_tcb(func);
    self.schedule_tcb(new);    
  }
  
  fn schedule_tcb(&mut self, tcb: Tcb) {
    cpu::current_cpu().disable_interrupts();
    
    self.queue.push_back(tcb);
    
    cpu::current_cpu().enable_interrupts();
  }
  
  fn unschedule_current(&mut self) -> ! {
    let c = |_: Tcb| { None };
    self.do_and_unschedule(c);
    unreachable!();
  }
  
  fn do_and_unschedule<'a, F>(&mut self, mut do_something: F) where F : FnMut(Tcb) -> Option<&'a mut Tcb> {
    debug!("unscheduling");
    
    cpu::current_cpu().disable_interrupts();
    
    let mut empty = Tcb { context: Context::empty() };
    let save_into = match do_something(self.queue.pop_front().unwrap()) {
        Some(tcb) => tcb,
        None => &mut empty
    };
    
    let next = self.queue.pop_back().unwrap();
    self.queue.push_front(next);
    
    Context::swap(&mut save_into.context, &self.queue.front().unwrap().context);
    
    cpu::current_cpu().enable_interrupts();
  }
  
  pub fn switch(&mut self) {
    cpu::current_cpu().disable_interrupts();
    
    if self.queue.len() == 1 {
        return;
    }
    let old = self.queue.pop_front().unwrap();
    let next = self.queue.pop_back().unwrap();
    self.queue.push_front(next);
    self.queue.push_back(old);
    
    let back: *mut Context = &mut self.queue.back_mut().unwrap().context;
    let front = self.queue.front().unwrap();
    Context::swap(unsafe { back.as_mut().unwrap() }, &front.context);
    
    unsafe { cpu::current_cpu().enable_interrupts(); } // TODO(ryan): make a mutex as enabling/disabling interrupts
  }
  
}

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

pub struct Mutex {
    taken: UnsafeCell<bool>,
    sleepers: UnsafeCell<LinkedList<Tcb>>
}

impl Mutex {

    pub const fn new() -> Mutex {
        Mutex { taken: UnsafeCell::new(false), sleepers: UnsafeCell::new(new_linked_list!())}
    }

    pub unsafe fn lock(&self) {
        cpu::current_cpu().disable_interrupts();
        while *self.taken.get() {
            get_scheduler().do_and_unschedule(&|me: Tcb| { 
                (*self.sleepers.get()).push_back(me);
                Some((*self.sleepers.get()).back_mut().unwrap())
            });
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
        match (*self.sleepers.get()).pop_front() {
            Some(tcb) => get_scheduler().schedule_tcb(tcb),
            None => ()
        }
        cpu::current_cpu().enable_interrupts();
    }
    
    pub unsafe fn destroy(&self) {
    }

}

unsafe impl Send for Condvar {}
unsafe impl Sync for Condvar {}

pub const CONDVAR_INIT: Condvar = Condvar { sleepers: UnsafeCell::new(new_linked_list!()) };

pub struct Condvar {
    sleepers: UnsafeCell<LinkedList<Tcb>>
}

impl Condvar {

    pub const fn new() -> Condvar {
        unsafe { Condvar { sleepers: UnsafeCell::new(new_linked_list!()) } }
    }

    pub unsafe fn notify_one(&self) {
        cpu::current_cpu().disable_interrupts();
        match (*self.sleepers.get()).pop_front() {
            Some(tcb) => get_scheduler().schedule_tcb(tcb),
            None => ()
        }
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
        get_scheduler().do_and_unschedule(&|me: Tcb| { 
            (*self.sleepers.get()).push_back(me);
            Some((*self.sleepers.get()).back_mut().unwrap())
        });
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
    s.unschedule_current(); 
  }
}

pub fn thread_stuff() {
  debug!("starting thread test");
  unsafe {
    let s: &mut Scheduler = get_scheduler();

    debug!("orig sched 0x{:x}", transmute_copy::<_, u32>(&s));
    //loop {};
    let t = || { test_thread() };
    s.schedule(box t);
    debug!("schedule okay");
    s.switch();
    debug!("back");
  }
}
