use core::prelude::*;

use alloc::boxed::Box;
use vec::Vec;

use io::{Write, Error};

pub trait Driver {

  fn init(&mut self);

}

pub trait DriverManager {

  fn get_drivers(&mut self) -> Vec<Box<NetworkDriver + 'static>>;

}

pub trait NetworkDriver: Driver {

  fn address(&mut self) -> [u8; 6];

  fn put_frame(&mut self, buf: &[u8]) -> Result<usize, Error>;
  // TODO(ryan): more
}

impl<'a> Write for NetworkDriver + 'a {

  fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
    match self.put_frame(buf) {
      Ok(_)  => Ok(buf.len()),
      Err(x) => Err(x)
    }
  }
  
  fn flush(&mut self) -> Result<(), Error> { Ok(()) }
}
