use prelude::v1::*;

use io::Write;
use self::multiboot::multiboot_info;
use self::arch::cpu;
use self::pci::Pci;
use self::driver::DriverManager;
use self::thread::Scheduler;
use fringe;

use ::alloc::arc::Arc;
use self::thread::Mutex;
use self::interrupt_handler::InterruptHandler;

mod interrupt_handler;

#[macro_use]
mod log;
pub mod arch;
mod terminal;
mod panic;
mod multiboot;
mod pci;
mod rtl8139;
mod driver;
mod keyboard;

pub mod args;
pub mod memchr;
pub mod net;
pub mod thread;

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

pub extern "C" fn main(magic: u32, info: usize) -> ! {
    // some preliminaries
    ::bump_ptr::set_allocator((15usize * 1024 * 1024) as *mut u8, (20usize * 1024 * 1024) as *mut u8);
    
    cpu::current_cpu().disable_interrupts();
    
    // we're going to now enter the scheduler to do the rest
    let bootstrapped_thunk = move || { 
        bootstrapped_main(magic, info as *mut multiboot_info); 
    };
    
    thread::start(bootstrapped_thunk);
    unreachable!();
}

fn bootstrapped_main(magic: u32, info: *mut multiboot_info) {
    debug!("kernel main thread start!");
    let handlers = Arc::new((0..0x30).map(|_| Arc::new(InterruptHandler::new())).collect::<Vec<_>>());

    unsafe {
        let mut c = cpu::current_cpu();
        
        let mut handlers_clone = handlers.clone();
        c.set_handler(box move |irq| { 
            let ref lock = handlers_clone[irq as usize];
            lock.notify();
        });

        cpu::current_cpu().enable_interrupts();

        test_allocator();
        
        
        if magic != multiboot::MULTIBOOT_BOOTLOADER_MAGIC {
            panic!("Multiboot magic is invalid");
        } else {
            debug!("Multiboot magic is valid. Info at 0x{:x}", info as u32);
            (*info).multiboot_stuff();
        }
        

        debug!("Going to interrupt: ");
        //cpu::current_cpu().test_interrupt();
        debug!("    back from interrupt!");


        pci_stuff();



        fringe_test();
        
        //scheduler::thread_stuff();
        
        
        make_keyboard(handlers[cpu::IRQ::PS2Keyboard as usize].clone());
        
        info!("Kernel main thread is done!");
        loop {
          thread::yield_now();
        }
  }
}


fn make_keyboard(handler: Arc<InterruptHandler>) {
    let func = move || { keyboard::Keyboard::new(cpu::Port::new(0x64), cpu::Port::new(0x60), handler).run() };
    //let f2: Box<FnBox() + Send> = unsafe { ::core::mem::transmute(func) };
    
    unsafe { self::thread::spawn(func, 1024*1024) };
}

fn fringe_test() {
  let mut bytes: [u8; 5000] = [0; 5000];

  let stack = fringe::SliceStack::new(&mut bytes);
  
  unsafe {
    let mut gen = fringe::Generator::unsafe_new(stack, move |yielder, ()| {
        for i in 1..4 { yielder.suspend(i) }
    });
    
    info!("{:?}", gen.resume(())); // Some(1)
    info!("{:?}", gen.resume(())); // Some(2)
    info!("{:?}", gen.resume(())); // Some(3)
    info!("{:?}", gen.resume(())); // None
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

pub fn init() {
    unimplemented!();
}
