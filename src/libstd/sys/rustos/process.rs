use prelude::v1::*;

use ffi::{OsString, OsStr, CString, CStr};
use fmt;
use io::{self, Error, ErrorKind};
use sys::pipe::AnonPipe;
use libc::c_int;

pub type RawStdio = u32;

#[derive(Clone)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>
}

impl Command {
    pub fn new(program: &OsStr) -> Command {
        unimplemented!();
    }

    pub fn arg(&mut self, arg: &OsStr) {
        unimplemented!();
    }
    pub fn args<'a, I: Iterator<Item = &'a OsStr>>(&mut self, args: I) {
        unimplemented!();
    }
    pub fn env(&mut self, key: &OsStr, val: &OsStr) {
        unimplemented!();
    }
    pub fn env_remove(&mut self, key: &OsStr) {
        unimplemented!();
    }
    pub fn env_clear(&mut self) {
        unimplemented!();
    }
    pub fn cwd(&mut self, dir: &OsStr) {
        unimplemented!();
    }
}

////////////////////////////////////////////////////////////////////////////////
// Processes
////////////////////////////////////////////////////////////////////////////////

/// Unix exit statuses
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ExitStatus {
    /// Normal termination with an exit code.
    Code(i32),

    /// Termination by signal, with the signal number.
    ///
    /// Never generated on Windows.
    Signal(i32),
}

impl ExitStatus {
    pub fn success(&self) -> bool {
        unimplemented!();
    }
    pub fn code(&self) -> Option<i32> {
        unimplemented!();
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

/// The unique id of the process (this should never be negative).
pub struct Process;

pub enum Stdio {
    Inherit,
    None,
    Raw(c_int),
}


impl Process {
    pub unsafe fn kill(&self) -> io::Result<()> {
        unimplemented!();
    }

    pub fn spawn(cfg: &Command,
                 in_fd: Stdio,
                 out_fd: Stdio,
                 err_fd: Stdio) -> io::Result<Process> {
        unimplemented!();
    }

    pub fn id(&self) -> u32 {
        unimplemented!();
    }

    pub fn wait(&self) -> io::Result<ExitStatus> {
        unimplemented!();
    }

    pub fn try_wait(&self) -> Option<ExitStatus> {
        unimplemented!();
    }
}
