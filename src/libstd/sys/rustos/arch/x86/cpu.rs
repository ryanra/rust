use core::prelude::*;
use core::cell::UnsafeCell;
use core::mem::transmute;

use io::{self, Read, Write, Error};

use super::idt::IDT;
use super::gdt::GDT;

// TODO remove box hack. It says it has a global destructor but I don't know why
lazy_static! {
  pub static ref CURRENT_CPU: UnsafeCell<CPU> = UnsafeCell::new(unsafe { CPU::new() });
}

pub fn current_cpu() -> &'static mut CPU {
    unsafe { transmute(CURRENT_CPU.get()) }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
#[repr(u8)]
pub enum IRQ { // after remap
  Timer        = 0x20,
  PS2Keyboard  = 0x21,
  Cascade      = 0x22,
  COM2         = 0x23,
  COM1         = 0x24,
  LPT2         = 0x25,
  Floppy       = 0x26,
  LPT1         = 0x27,
  CmosClock    = 0x28,
  FreeOne      = 0x29,
  FreeTwo      = 0x2a,
  FreeThree    = 0x2b,
  PsMouse      = 0x2c,
  FPU          = 0x2d,
  PrimaryAta   = 0x2e,
  SecondaryAta = 0x2f
}

fn u8_to_irq(x: u8) -> Option<IRQ> {
    if x >= IRQ::Timer as u8 && x <= IRQ::SecondaryAta as u8 {
        Some(unsafe { transmute(x) })
    } else {
        None
    }
}

extern "C" {

  fn interrupt();

  fn debug(s: &str, u: u32);

}

#[allow(dead_code)]
pub struct CPU {
  gdt: GDT,
  idt: IDT,
  handler: Option<Box<Fn(IRQ)>>,
  //ports: Ports
}

impl CPU {

  pub unsafe fn new() -> CPU {
    let mut gdt = GDT::new();

    gdt.identity_map();
    gdt.enable();

    PIC::master().remap_to(0x20);
    PIC::slave().remap_to(0x28);

    let mut idt = IDT::new();

    idt.enable();
    CPU { gdt: gdt, idt: idt, handler: None }
  }

  pub fn set_handler(&mut self, f: Box<Fn(IRQ)>) {
    self.handler = Some(f);
  }
  
  pub fn handle(&mut self, interrupt_number: u8) {
    self.disable_interrupts();
    //if false {
        match self.handler {
            Some(ref f) => {
                match u8_to_irq(interrupt_number) {
                    Some(irq) => { f(irq) },
                    None => (),//warn!("Unkonwn IRQ {}", interrupt_number),
                }
            },
            None => (), //warn!("No interrupt handler set!"),
        }
    //}
    self.acknowledge_irq(interrupt_number);
    self.enable_interrupts();
  }

  pub unsafe fn register_irq(&mut self, irq: IRQ, handler: extern "C" fn () -> ()) {
    self.idt.add_entry(irq as u32, handler);
  }

  pub fn idle(&mut self) {
    unsafe { asm!("hlt" :::: "volatile") }
  }

  fn acknowledge_irq(&mut self, interrupt_number: u8) {
    PIC::master().control_port.out_b(interrupt_number); //TODO(ryan) ugly and only for master PIC
  }

  pub fn enable_interrupts(&mut self) {
    unsafe { IDT::enable_interrupts(); }
  }

  pub fn disable_interrupts(&mut self) {
    IDT::disable_interrupts();
  }

  pub unsafe fn test_interrupt(&mut self) {
    interrupt();
  }
  
  pub fn interrupts_enabled(&mut self) -> bool {
    IDT::interrupts_enabled()
  }
  
  pub fn interrupt_lock(&mut self) -> InterruptLock {
    let already_enabled = IDT::interrupts_enabled();

    self.disable_interrupts();
    return InterruptLock{ should_enable: already_enabled }
  }

}

struct InterruptLock {
    should_enable: bool,
}

impl Drop for InterruptLock {
    
    fn drop(&mut self) {
        if self.should_enable {
            info!("should_enable is true: enabling ints");
            current_cpu().enable_interrupts();
        }
    }
    
}

pub extern "C" fn unified_handler(interrupt_number: u32) {
  current_cpu().handle(interrupt_number as u8);
}

pub extern "C" fn add_entry(idt: &mut IDT, index: u32, f: unsafe extern "C" fn() -> ()) {
  idt.add_entry(index, f);
}


struct PIC {
  control_port: Port,
  mask_port: Port,
  is_master: bool
}

impl PIC {

  fn master() -> PIC {
    PIC { control_port: Port::new(0x20), mask_port: Port::new(0x21), is_master: true}
  }

  fn slave() -> PIC {
    PIC { control_port: Port::new(0xA0), mask_port: Port::new(0xA1), is_master: false}
  }

  unsafe fn remap_to(&mut self, start: u8) {
    let icw1 = 0x11;
    let icw4 = 0x1;
    let enable_all = 0x00;
    let typ = if self.is_master { 0x2 } else { 0x4 };

    self.control_port.out_b(icw1);
    self.mask_port.write(&[start, typ, icw4, enable_all]).ok();
  }

}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Port {
  port_number: u16
}

impl Port {

  pub fn new(number: u16) -> Port {
    Port { port_number: number }
  }

  pub fn in_b(&mut self) -> u8 {
    let mut ret: u8;
    unsafe {
      asm!("inb $1, $0" : "={al}"(ret) :"{dx}"(self.port_number) ::)
    }
    return ret;
  }

  pub fn out_b(&mut self, byte: u8) {
    unsafe {
      asm!("outb $1, $0" :: "{dx}"(self.port_number), "{al}"(byte) :: "volatile")
    }
  }

  pub fn out_w(&mut self, word: u16) {
    unsafe {
      asm!("outw $1, $0" ::"{dx}"(self.port_number), "{ax}"(word) ::)
    }
  }

  pub fn in_w(&mut self) -> u16 {
    let mut ret: u16;
    unsafe {
      asm!("inw $1, $0" : "={ax}"(ret) :"{dx}"(self.port_number)::)
    }
    ret
  }

  pub fn out_l(&mut self, long: u32) {
    unsafe {
      asm!("outl $1, $0" ::"{dx}"(self.port_number), "{eax}"(long) ::)
    }
  }

  pub fn in_l(&mut self) -> u32 {
    let mut ret: u32;
    unsafe {
      asm!("inl $1, $0" : "={eax}"(ret) :"{dx}"(self.port_number)::)
    }
    ret
  }

  pub fn io_wait() {
    Port::new(0x80).out_b(0);
  }

}

impl io::Read for Port
{
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
    for el in buf.iter_mut() {
      *el = self.in_b();
    }
    Ok(buf.len())
  }

}

impl io::Write for Port
{
  fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
    for &byte in buf.iter() {
      self.out_b(byte);
    }
    Ok(buf.len())
  }

  fn flush(&mut self) -> Result<(), Error> {
    Ok(())
  }
}
