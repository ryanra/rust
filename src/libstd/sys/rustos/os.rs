use prelude::v1::*;

use core::slice;
use fmt;
use io;
use iter;
use ffi::{CString, CStr, OsString, OsStr};
use path::{self, PathBuf};
use error::Error as StdError;

/// Returns the platform-specific value of errno
pub fn errno() -> i32 {
    unimplemented!();
}

/// Gets a detailed string description for the given error number.
pub fn error_string(errno: i32) -> String {
    unimplemented!();
}

pub fn getcwd() -> io::Result<PathBuf> {
    unimplemented!();
}

pub fn chdir(p: &path::Path) -> io::Result<()> {
    unimplemented!();
}

pub struct SplitPaths<'a> {
    iter: iter::Map<slice::Split<'a, u8, fn(&u8) -> bool>,
                    fn(&'a [u8]) -> PathBuf>,
}

pub fn split_paths<'a>(unparsed: &'a OsStr) -> SplitPaths<'a> {
    unimplemented!();
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> { unimplemented!(); }
    fn size_hint(&self) -> (usize, Option<usize>) { unimplemented!(); }
}

#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(paths: I) -> Result<OsString, JoinPathsError>
    where I: Iterator<Item=T>, T: AsRef<OsStr>
{
    unimplemented!();
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

impl StdError for JoinPathsError {
    fn description(&self) -> &str { unimplemented!(); }
}

pub fn current_exe() -> io::Result<PathBuf> {
    unimplemented!();
}

pub struct Args;

impl Iterator for Args {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> { unimplemented!(); }
    fn size_hint(&self) -> (usize, Option<usize>) { unimplemented!(); }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize { unimplemented!(); }
}

pub fn args() -> Args {
    unimplemented!();
}

pub struct Env;

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> { unimplemented!(); }
    fn size_hint(&self) -> (usize, Option<usize>) { unimplemented!(); }
}

/// Returns a vector of (variable, value) byte-vector pairs for all the
/// environment variables of the current process.
pub fn env() -> Env {
    unimplemented!();
}

pub fn getenv(k: &OsStr) -> io::Result<Option<OsString>> {
    unimplemented!();
}

pub fn setenv(k: &OsStr, v: &OsStr) -> io::Result<()> {
    unimplemented!();
}

pub fn unsetenv(n: &OsStr) -> io::Result<()> {
    unimplemented!();
}

pub fn page_size() -> usize {
    unimplemented!();
}

pub fn temp_dir() -> PathBuf {
    unimplemented!();
}

pub fn home_dir() -> Option<PathBuf> {
    unimplemented!();
}

pub fn exit(code: i32) -> ! {
    unimplemented!();
}
