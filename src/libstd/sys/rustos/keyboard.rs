use core::prelude::*;

use super::arch::cpu::Port;

static KEY_CODE_TO_ASCII: &'static [u8] = b"??1234567890-=??qwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?"; 

pub struct Keyboard {
  control_port: Port,
  data_port: Port,
  wait: Box<FnMut()>,
}

impl Keyboard {

  pub fn new(control_port: Port, data_port: Port, wait: Box<FnMut()>) -> Keyboard {
    Keyboard { control_port: control_port, data_port: data_port, wait: wait }
  }
  
  pub fn run(mut self) {
    info!("keyboard starting");
    loop {
        (self.wait)();
        let keycode = self.data_port.in_b();
        match KEY_CODE_TO_ASCII.get(keycode as usize) {
        Some(ascii) => {
            __print!("{}", *ascii as char);
        },
        None => ()
        }
    }
  }
    
}

unsafe impl Send for Keyboard {}