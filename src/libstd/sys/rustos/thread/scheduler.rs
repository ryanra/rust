use core::prelude::*;
use core::cell::{UnsafeCell, Cell};
use core::mem::{transmute, transmute_copy};
use core::ptr;
use io;
use time::Duration;

use core_collections::String;
use core_collections::btree_map::BTreeMap;

use alloc::boxed::{Box, FnBox};
use alloc::arc::Arc;

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
  group: fringe_wrapper::Group<'static, ThreadResponse, ThreadRequest, fringe::OwnedStack>,
  name: String,
  posix_local: BTreeMap<Key, *mut u8>,  // TODO(ryan): implement dtor calling in drop
}

impl Tcb {
    
    fn new<F>(func: F, stack_size: usize) -> Tcb where F: FnOnce() + Send + 'static {
        let stack = fringe::OwnedStack::new(stack_size);
        unsafe {
            Tcb { group: fringe_wrapper::Group::new(func, stack), name: String::new(), posix_local: BTreeMap::new() }
        }
    }
    
}

unsafe impl Send for Tcb {}

// Request of thread to scheduler
enum ThreadRequest {
    Yield,
    StageUnschedule,                // Request to be unscheduled and get a Node container Tcb
    Schedule(Box<Node<Tcb>>),  // Schedule a Tcb
    CompleteUnschedule,                    // After unscheduling self, must send this message
    Interrupted(cpu::IRQ),     // tell scheduler that thread has been interrupted by a cpu interrupt
}

// Response
enum ThreadResponse {
    Nothing,
    Unscheduled(Box<Node<Tcb>>),
    HandleIRQ(cpu::IRQ), // Tell thread to handle the given IRQ
}

// Notes that the scheduler thread cannot do any allocations (because
// eventually allocations will require a lock which will be implemented
// with the scheduler itself). Further, the locks cannot do any allocations
// themselves because it causes a cyclic dependency with the allocator.
// The solution is to pass around Box<Node<Tcb>>'s which have their
// memory pre-allocated to them.
pub struct Scheduler {
  queue: LinkedList<Tcb>,
  interrupt_handler: Box<InterruptHandler>,
  interrupt_handler_thread: Tcb,
}

const STACK_SIZE: usize = 1024*1024;

impl Scheduler {
  
  fn new() -> Scheduler {
    // TODO(ryan): This is kinda gross, fix it
    let handler_raw = Box::into_raw(box InterruptHandler::new());
    let handler = unsafe { Box::from_raw(handler_raw) };
    let handler_clone = unsafe { Box::from_raw(handler_raw) };
    let mut s = Scheduler { queue: LinkedList::new(),
                            interrupt_handler: handler,
                            interrupt_handler_thread: Tcb::new(move || {unsafe { handler_clone.run() }}, STACK_SIZE)
    }; 
    s
  }
  
  fn request(&mut self, request: ThreadRequest) -> ThreadResponse {
    debug!("suspending");
    cpu::current_cpu().disable_interrupts();
    let response = unsafe { self.current_tcb().group.suspend(request) };
    cpu::current_cpu().enable_interrupts();
    response
  }
  
  fn do_and_unschedule<F>(&mut self, f: F) where F: FnOnce(Box<Node<Tcb>>) {
    // TODO(ryan): determine if it's okay for this code to run in current
    // thread rather than in scheduler
    let my_tcb = match self.request(ThreadRequest::StageUnschedule) {
        ThreadResponse::Unscheduled(x) => x,
        _ => unreachable!(),
    };
    f(my_tcb);
    self.request(ThreadRequest::CompleteUnschedule);
  }
  
  fn current_tcb(&self) -> &Tcb {
    self.queue.front().unwrap()
  }
  
  fn current_tcb_mut(&mut self) -> &mut Tcb {
    self.queue.front_mut().unwrap()
  }
  
  pub fn wait(&mut self, irq: cpu::IRQ) {
    self.interrupt_handler.wait(irq);
  }
  
  pub fn bootstrap_start<F>(f: F) -> ! where F: FnOnce() + Send + 'static {
    get_scheduler().run(Self::new_tcb(f, STACK_SIZE))
  }
  
  fn run(&mut self, start: Box<Node<Tcb>>) -> ! {
    // scheduler now takes control of the CPU
    self.queue.push_front_node(Self::new_tcb(Self::idle, STACK_SIZE));
    self.queue.push_front_node(start);

    let mut response = ThreadResponse::Nothing;
    loop {
        let request = unsafe { 
            self.queue.front_mut().unwrap().group.resume(response) 
        };
        debug!("got request");
        response = match request {
            Some(req) => match req {
                ThreadRequest::Yield => {
                    debug!("Requesting yield");
                    let current = self.queue.pop_front_node().unwrap();
                    self.queue.push_back_node(current);
                    ThreadResponse::Nothing
                },
                ThreadRequest::StageUnschedule => {
                    debug!("Requesting Unschedule");
                    // We have to pass `node` to a resume call on the tcb in node.
                    // To do so, we need to get around the borrow checker.
                    let mut node: &Box<Node<Tcb>> = self.queue.front_node().unwrap();
                    let node_as_int: *const usize = unsafe { ::core::mem::transmute(node) };                        
                    unsafe { ThreadResponse::Unscheduled(transmute(*node_as_int)) }
                },
                ThreadRequest::CompleteUnschedule => {
                    debug!("Completing unschedule");
                    // We can assert that last response was unscheduled
                    // Finish unscheduling. Tcb's ownership has already been passed
                    ::core::mem::forget(self.queue.pop_front_node());
                    ThreadResponse::Nothing
                },
                ThreadRequest::Schedule(tcb_node) => {
                    debug!("Requesting schedule");
                    self.queue.push_back_node(tcb_node);
                    ThreadResponse::Nothing
                },
                ThreadRequest::Interrupted(irq) => {
                    unsafe { self.interrupt_handler_thread.group.resume(ThreadResponse::HandleIRQ(irq)); }
                    ThreadResponse::Nothing
                }
            },
            None => {
                // Thread is finished.
                // TODO(ryan): this will call the allocator. Fix so it doesn't.
                self.queue.pop_front_node();
                ThreadResponse::Nothing
            },
        }
        
    }
  }
  
  fn idle() {
    get_scheduler().request(ThreadRequest::Yield);
    loop {
        // TODO should idle and yield in here...
    }
  }
  
  fn new_tcb<F>(func: F, stack_size: usize) -> Box<Node<Tcb>> where F: FnOnce() + Send + 'static {
    box Node::new(Tcb::new(func, stack_size))
  }
  
}

pub struct InterruptHandler {
    mutex: Mutex,
    irq_waiters: BTreeMap<cpu::IRQ, (Mutex, Condvar)>,
}

impl InterruptHandler {

    pub fn new() -> InterruptHandler {
        InterruptHandler { mutex: Mutex::new(), irq_waiters: BTreeMap::new() }
    }
    
    pub fn thread_interrupted(irq: cpu::IRQ) {
        // Called when a thread has been interrupted
        
        // Idea is to yield to the scheduler. Scheduler will then
        // pass control to us in main_thread. Then, in main_thread,
        // we wake any sleepers.
        get_scheduler().request(ThreadRequest::Interrupted(irq));
    }
    
    // Wait on the given IRQ
    pub fn wait(&mut self, irq: cpu::IRQ) {
        unsafe {
            self.mutex.lock();
            if !self.irq_waiters.contains_key(&irq) {
                self.irq_waiters.insert(irq, (Mutex::new(), Condvar::new()));
            }
            let &(ref mutex, ref condvar) = match self.irq_waiters.get(&irq) {
                Some(hit) => hit,
                None => unreachable!(),
            };
            mutex.lock();
            condvar.wait(&mutex);
            mutex.unlock();
            self.mutex.unlock();
        }
    }
    
    unsafe fn run(&self) {
        loop {
            let next_interrupt: cpu::IRQ = match  get_scheduler().request(ThreadRequest::Yield) {
                ThreadResponse::HandleIRQ(irq) => irq,
                _ =>  unreachable!(),
            };
            self.mutex.lock();
            match self.irq_waiters.get(&next_interrupt) {
                Some(&(ref mutex, ref condvar)) => {
                    debug!("Waking sleepers on {:?}", next_interrupt);
                    mutex.lock();
                    condvar.notify_all();
                    mutex.unlock();
                },
                None => (),
            }
            self.mutex.unlock();
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
            get_scheduler().do_and_unschedule(|tcb_node| {
                (*self.sleepers.get()).push_back_node(tcb_node)
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
        match (*self.sleepers.get()).pop_front_node() {
            Some(tcb_node) => { get_scheduler().request(ThreadRequest::Schedule(tcb_node)); },
            None => (),
        }
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
        match (*self.sleepers.get()).pop_front_node() {
            Some(tcb_node) => { get_scheduler().request(ThreadRequest::Schedule(tcb_node)); },
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
        get_scheduler().do_and_unschedule(|tcb_node| {
            (*self.sleepers.get()).push_back_node(tcb_node)
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

pub struct ReentrantMutex {
    mutex: Mutex,
    holder: Cell<Option<*const Tcb>>,
}

impl ReentrantMutex {
    pub unsafe fn uninitialized() -> ReentrantMutex {
        ReentrantMutex { mutex: Mutex::new(), holder: Cell::new(None) }
    }
    
    pub fn init(&self) { }

    fn me(&self) -> *const Tcb {
        get_scheduler().current_tcb() as *const Tcb
    }
    
    fn has_lock(&self) -> bool {
        self.holder.get().map(|ptr| { ptr == self.me()}) == Some(true)
    }
    
    fn post_lock(&self) {
        assert!(self.holder.get().is_none());
        self.holder.set(Some(self.me()));
    }
    
    pub unsafe fn lock(&self) {        
        if !self.has_lock() {
            self.mutex.lock();
            self.post_lock();
        }
    }

    pub unsafe fn try_lock(&self) -> bool {
        let locked = self.has_lock() || self.mutex.try_lock();
        if locked {
            self.post_lock();
        }
        locked
    }

    pub unsafe fn unlock(&self) {
        assert!(self.has_lock());
        self.holder.set(None);
        self.mutex.unlock();
    }

    pub unsafe fn destroy(&self) {
    }
}

struct RunningBarrier {
    condvar: Condvar,
    mutex: Mutex,
    running: Cell<bool>,
}
    
unsafe impl Sync for RunningBarrier {}

pub struct Thread {
    barrier: Arc<RunningBarrier>,
}

unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

impl Thread {
    pub unsafe fn new<'a>(stack: usize, p: Box<FnBox() + 'a>)
                          -> io::Result<Thread> {
        let mut barrier = Arc::new(RunningBarrier {condvar: Condvar::new(), mutex: Mutex::new(), running: Cell::new(true) } );
        let mut barrier_clone = barrier.clone();
        let muted: Box<FnBox() + Send> = transmute(p);
        let f = move || {
            muted();
            barrier_clone.mutex.lock();
            barrier_clone.running.set(false);
            barrier_clone.condvar.notify_all();
            barrier_clone.mutex.unlock();
        };
        let tcb = Scheduler::new_tcb(f, stack);
        get_scheduler().request(ThreadRequest::Schedule(tcb));
        Ok(Thread { barrier: barrier } )
    }

    pub fn yield_now() {
        get_scheduler().request(ThreadRequest::Yield);
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub fn set_name(name: &str) {
        get_scheduler().current_tcb_mut().name.clear();
        get_scheduler().current_tcb_mut().name.push_str(name);
    }

    pub fn sleep(dur: Duration) {
        unimplemented!();
    }

    pub fn join(self) {
        unsafe {
            self.barrier.mutex.lock();
            while self.barrier.running.get() {
                self.barrier.condvar.wait(&self.barrier.mutex);
            }
            self.barrier.mutex.unlock();
        }
    }
}

pub type Key = usize;

pub struct KeyInner {
    dtor: Option<unsafe extern fn(*mut u8)>,
}

#[inline]
pub unsafe fn create(dtor: Option<unsafe extern fn(*mut u8)>) -> Key {
    Box::into_raw(Box::new(KeyInner { dtor: dtor })) as usize
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    get_scheduler().current_tcb_mut().posix_local.insert(key, value);
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    match get_scheduler().current_tcb().posix_local.get(&key) {
        Some(val) => *val,
        None => 0 as *mut u8,
    }
}

#[inline]
pub unsafe fn destroy(key: Key) {
    ::core::mem::drop(Box::from_raw(key as *mut KeyInner))
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
    s.request(ThreadRequest::Schedule(Scheduler::new_tcb(t, STACK_SIZE)));
    debug!("schedule okay");
    s.request(ThreadRequest::Yield);
    debug!("back");
  }
}
