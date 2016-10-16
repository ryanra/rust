use core::prelude::*;

use alloc::boxed::Box;
use vec::Vec;

use io::{Write, Error};

#[stable(feature = "rustos", since = "0.0.1")]
pub trait Driver {

  #[stable(feature = "rustos", since = "0.0.1")]
  fn init(&mut self);

}

#[stable(feature = "rustos", since = "0.0.1")]
pub trait DriverManager {

  fn get_drivers(&mut self) -> Vec<Box<NetworkDriver + 'static>>;

}

#[stable(feature = "rustos", since = "0.0.1")]
pub trait NetworkDriver: Driver {

  #[stable(feature = "rustos", since = "0.0.1")]
  fn address(&mut self) -> [u8; 6];

  #[stable(feature = "rustos", since = "0.0.1")]
  fn put_frame(&mut self, buf: &[u8]) -> Result<usize, Error>;
  // TODO(ryan): more
}

#[stable(feature = "rustos", since = "0.0.1")]
impl<'a> Write for NetworkDriver + 'a {


  fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
    match self.put_frame(buf) {
      Ok(_)  => Ok(buf.len()),
      Err(x) => Err(x)
    }
  }
  
  fn flush(&mut self) -> Result<(), Error> { Ok(()) }
}
