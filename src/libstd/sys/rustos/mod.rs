use prelude::v1::*;

use io::Write;
use self::multiboot::multiboot_info;
use self::arch::cpu;
use self::pci::Pci;
use self::driver::DriverManager;
use self::thread::scheduler;

#[macro_use]
mod log;
pub mod arch;
mod terminal;
mod panic;
mod multiboot;
mod pci;
mod rtl8139;
mod driver;
pub mod net;
pub mod thread;
pub mod time;
pub mod stdio;
pub mod pipe;
pub mod os_str;
pub mod backtrace;
pub mod fs;
pub mod process;
pub mod os;
pub mod stack_overflow;

pub mod mutex {
    pub use super::thread::scheduler::{Mutex, ReentrantMutex};
}

pub mod rwlock {
    pub use super::thread::scheduler::{RWLock};
}

pub mod condvar {
    pub use super::thread::scheduler::{Condvar};
}

pub mod thread_local {
    pub use super::thread::scheduler::{Key, create, set, get, destroy};
}

fn test_allocator() {
  let mut v = Vec::new();

  debug!("Testing allocator with a vector push");
  v.push("   hello from a vector!");
  debug!("   push didn't crash");
  match v.pop() {
    Some(string) => debug!("{}", string),
    None => debug!("    push was weird...")
  }

}

fn put_char(c: u8) {
  __print!("{}", c as char);
}

pub extern "C" fn main(magic: u32, info: *mut u8) -> ! {
    // some preliminaries
    ::bump_ptr::set_allocator((15usize * 1024 * 1024) as *mut u8, (20usize * 1024 * 1024) as *mut u8);
    let mut c = cpu::current_cpu();
    unsafe { c.enable_interrupts(); }
        
    // we're going to now enter the scheduler to do the rest
    let bootstrapped_thunk = move || { 
        bootstrapped_main(magic, info as *mut multiboot_info); 
    };
    
    scheduler::get_scheduler().schedule(box bootstrapped_thunk);
    scheduler::get_scheduler().bootstrap_start(); // okay, scheduler, take it away!
    unreachable!();
}

fn bootstrapped_main(magic: u32, info: *mut multiboot_info) {
    unsafe {
        let mut c = cpu::current_cpu();
        unsafe { c.enable_interrupts(); }
        c.make_keyboard(put_char);
        
        debug!("kernel main thread start!");

        test_allocator();
        
        
        if magic != multiboot::MULTIBOOT_BOOTLOADER_MAGIC {
            panic!("Multiboot magic is invalid");
        } else {
            debug!("Multiboot magic is valid. Info at 0x{:x}", info as u32);
            (*info).multiboot_stuff();
        }
        
        
        debug!("Going to interrupt: ");
        cpu::current_cpu().test_interrupt();
        debug!("    back from interrupt!");
        
        pci_stuff();
        
        scheduler::thread_stuff();
        
        info!("Kernel main thread is done!");
  }
}

fn pci_stuff() {
  let mut pci = Pci::new();
  pci.init();
  let mut drivers = pci.get_drivers();
  debug!("Found drivers for {} pci devices", drivers.len());
  match drivers.pop() {
    Some(mut driver) => {
      driver.init();
      net::NetworkStack::new(driver).test().ok();
    }
    None => ()
  }

}

#[no_mangle]
pub extern "C" fn debug(s: &'static str, u: u32) {
  debug!("{} 0x{:x}", s, u)
}

pub extern "C" fn __morestack() {
  unreachable!("__morestack");
}

pub extern "C" fn abort() -> ! {
  loop {}
}

pub extern "C" fn callback() {
  debug!("    in an interrupt!");
}

// TODO(ryan): figure out what to do with these:
/*
#[lang = "stack_exhausted"]
extern fn stack_exhausted() {}
*/


// for deriving
//#[doc(hidden)]
//mod std {
//  pub use core::*;
//}
use io::{self, ErrorKind};
use libc;

pub fn decode_error_kind(errno: i32) -> ErrorKind {
    match errno as libc::c_int {
        libc::ECONNREFUSED => ErrorKind::ConnectionRefused,
        libc::ECONNRESET => ErrorKind::ConnectionReset,
        libc::EPERM | libc::EACCES => ErrorKind::PermissionDenied,
        libc::EPIPE => ErrorKind::BrokenPipe,
        libc::ENOTCONN => ErrorKind::NotConnected,
        libc::ECONNABORTED => ErrorKind::ConnectionAborted,
        libc::EADDRNOTAVAIL => ErrorKind::AddrNotAvailable,
        libc::EADDRINUSE => ErrorKind::AddrInUse,
        libc::ENOENT => ErrorKind::NotFound,
        libc::EINTR => ErrorKind::Interrupted,
        libc::EINVAL => ErrorKind::InvalidInput,
        libc::ETIMEDOUT => ErrorKind::TimedOut,
        libc::consts::os::posix88::EEXIST => ErrorKind::AlreadyExists,

        // These two constants can have the same value on some systems,
        // but different values on others, so we can't use a match
        // clause
        x if x == libc::EAGAIN || x == libc::EWOULDBLOCK =>
            ErrorKind::WouldBlock,

        _ => ErrorKind::Other,
    }
}


pub fn init() {
    unimplemented!();
}



pub fn ms_to_timeval(ms: u64) -> libc::timeval {
    libc::timeval {
        tv_sec: (ms / 1000) as libc::time_t,
        tv_usec: ((ms % 1000) * 1000) as libc::suseconds_t,
    }
}

