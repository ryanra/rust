use io;
use libc;

pub struct AnonPipe;

pub fn anon_pipe() -> io::Result<(AnonPipe, AnonPipe)> {
    unimplemented!();
}

impl AnonPipe {
    pub fn from_fd(fd: libc::c_int) -> AnonPipe {
        unimplemented!();
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!();
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        unimplemented!();
    }

    pub fn raw(&self) -> libc::c_int { unimplemented!(); }
    
}
