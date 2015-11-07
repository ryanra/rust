use core::mem::transmute;

use alloc::boxed::Box;

#[allow(dead_code)]
#[repr(packed)]
pub struct Context {
  regs: Registers,
  esp: u32,
  instruction_pointer: *mut u8,
  stack: Box<[u8]>
}

#[allow(dead_code)]
#[repr(packed)]
struct Registers {
    regs: [u32; 7]
}

extern "C" {
  
  pub fn save_context(t: &mut Context) -> bool;
  
  pub fn restore_context(t: &Context);
  
}

impl Context {

  fn empty_regs() -> Registers {
    Registers { regs: [0, 0, 0, 0, 0, 0, 0] }
  }
  
  pub fn empty() -> Context {
    unsafe {
        Context { stack: transmute::<u64, Box<[u8]>>(0_u64), instruction_pointer: transmute(0usize), regs: Context::empty_regs(), esp: 0}
    }
  }

  pub fn new(func: extern "C" fn() -> (), stack: Box<[u8]>, esp: usize) -> Context {
    unsafe {
      let mut t = Context::empty();
      save_context(&mut t);
      t.esp = esp as u32;
      t.stack = stack;
      t.instruction_pointer = transmute(func);
      debug!("new thread:");
      t.debug();
      t
    }
  }
  
  pub fn debug(&self) {
    unsafe { debug!("   self is 0x{:x}", transmute::<&Context, u32>(self)) }
    debug!("   eip is 0x{:x}", self.instruction_pointer as u32);
    debug!("   esp is 0x{:x}", self.esp as u32);
  }
   
}
