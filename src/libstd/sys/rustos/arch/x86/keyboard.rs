use core::prelude::*;

use super::cpu::Port;

static KEY_CODE_TO_ASCII: &'static [u8] = b"??1234567890-=??qwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?"; 

#[derive(Copy, Clone)]
pub struct Keyboard {
  callback: fn (u8) -> (),
  control_port: Port,
  data_port: Port
}

impl Keyboard {

  pub fn new(callback: fn (u8) -> (), control_port: Port, data_port: Port) -> Keyboard {
    Keyboard { callback: callback, control_port: control_port, data_port: data_port }
  }
  
  pub fn register_callback(&mut self, callback: fn (u8) -> ()) {
    self.callback = callback;
  }
  
  pub fn got_interrupted(&mut self) {
    let keycode = self.data_port.in_b();
    match KEY_CODE_TO_ASCII.get(keycode as usize) {
      Some(ascii) => {
	let func = self.callback;
	func(*ascii);
      },
      None => ()
    }
  }
    
}
