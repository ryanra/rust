use core::prelude::*;
use core::cell::UnsafeCell;

use super::arch::vga;

// TODO(john): next line is still breaking abstractions (but I can't
// find a nice way to init it either...)
lazy_static! {
    static ref GLOBAL: UnsafeCell<Terminal> = UnsafeCell::new(Terminal::new());
}


pub fn get_terminal() -> &'static mut Terminal {
    unsafe { ::core::mem::transmute(GLOBAL.get()) } 
}

struct Point(usize, usize);

pub struct Terminal {
  current: Point,
  vga:     *mut vga::Buffer
}

impl Terminal
{
  fn new() -> Terminal {
    Terminal {
        current: Point(0,0),
        vga: unsafe { vga::GLOBAL.get() }
    }
  }

  fn get_vga_mut(&mut self) -> &mut vga::Buffer {
    unsafe { &mut *self.vga }
  }

  fn put_char(&mut self, c: u8) {
    if c == '\n' as u8 {
      self.current = Point(0, self.current.1 + 1);
    } else {
      self.get_vga_mut()[self.current.1][self.current.0] =
        vga::Entry::new(c, vga::Color::White, vga::Color::Black);
      self.current.0 += 1;
    }

    // line wrap
    if self.current.0 >= vga::X_MAX {
      self.current.0 = 0;
      self.current.1 += 1;
    }

    if self.current.1 >= vga::Y_MAX {
      self.scroll();
      self.current.1 = vga::Y_MAX - 1;
    }
  }


  fn scroll(&mut self)
  {
    let mut rows = self.get_vga_mut().iter_mut();

    let mut n     = rows.next().unwrap();
    let mut suc_n = rows.next();

    while let Some(b) = suc_n {
      ::core::mem::swap(n, b); // TODO(john) just need to copy b -> a
      n = b;
      suc_n = rows.next();
    }
    unsafe {
      *n = ::core::mem::zeroed(); // last row
    }
  }

  pub fn clear_screen(&mut self) {
    unsafe {
      *self.get_vga_mut() = ::core::mem::zeroed();
    }
  }
  
}

impl Drop for Terminal {

    fn drop(&mut self) {
        debug!("dropping term!");
    }

}

impl ::io::Write for Terminal {

  fn write(&mut self, buf: &[u8]) -> Result<usize, ::io::Error> {
    for &c in buf.iter() {
      self.put_char(c);
    }
    Ok(buf.len())
  }
  
  fn flush(&mut self) -> Result<(), ::io::Error> {
    Ok(())
  }
  
}
