use core::prelude::*;
use io::prelude::*;

use ffi::{CString, CStr, OsString, OsStr};
use fmt;
use io::{self, Error, SeekFrom};
use path::{Path, PathBuf};

pub struct File;

pub struct FileAttr;

pub struct ReadDir;

struct Dir;

unsafe impl Send for Dir {}
unsafe impl Sync for Dir {}

pub struct DirEntry;

#[derive(Clone)]
pub struct OpenOptions;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FilePermissions ;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct FileType;

pub struct DirBuilder;

pub fn copy(from: &Path, to: &Path) -> io::Result<u64> {
    unimplemented!();
}

impl FileAttr {
    pub fn size(&self) -> u64 { unimplemented!(); }
    pub fn perm(&self) -> FilePermissions {
         unimplemented!();
    }

    pub fn accessed(&self) -> u64 {
         unimplemented!();
    }
    pub fn modified(&self) -> u64 {
         unimplemented!();
    }

    pub fn file_type(&self) -> FileType {
         unimplemented!();
    }

}

impl FilePermissions {
    pub fn readonly(&self) -> bool {  unimplemented!(); }
    pub fn set_readonly(&mut self, readonly: bool) {
         unimplemented!();
    }
}

impl FileType {
    pub fn is_dir(&self) -> bool {  unimplemented!(); }
    pub fn is_file(&self) -> bool {  unimplemented!(); }
    pub fn is_symlink(&self) -> bool {  unimplemented!(); }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
         unimplemented!();
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
         unimplemented!();
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
         unimplemented!();
    }

    pub fn file_name(&self) -> OsString {
         unimplemented!();
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
         unimplemented!();
    }

    pub fn file_type(&self) -> io::Result<FileType> {
         unimplemented!();
    }

}

impl OpenOptions {
    pub fn new() -> OpenOptions {
         unimplemented!();
    }

    pub fn read(&mut self, read: bool) {
         unimplemented!();
    }

    pub fn write(&mut self, write: bool) {
         unimplemented!();
    }

    pub fn append(&mut self, append: bool) {
         unimplemented!();
    }

    pub fn truncate(&mut self, truncate: bool) {
         unimplemented!();
    }

    pub fn create(&mut self, create: bool) {
         unimplemented!();
    }

}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
         unimplemented!();
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
         unimplemented!();
    }

    pub fn fsync(&self) -> io::Result<()> {
         unimplemented!();
    }

    pub fn datasync(&self) -> io::Result<()> {
         unimplemented!();
    }

    pub fn truncate(&self, size: u64) -> io::Result<()> {
         unimplemented!();
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
         unimplemented!();
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
         unimplemented!();
    }

    pub fn flush(&self) -> io::Result<()> {  unimplemented!(); }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
         unimplemented!();
    }

}

impl DirBuilder {
    pub fn new() -> DirBuilder {
         unimplemented!();
    }

    pub fn mkdir(&self, p: &Path) -> io::Result<()> {
         unimplemented!();
    }

}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

pub fn readdir(p: &Path) -> io::Result<ReadDir> {
     unimplemented!();
}

pub fn unlink(p: &Path) -> io::Result<()> {
     unimplemented!();
}

pub fn rename(old: &Path, new: &Path) -> io::Result<()> {
     unimplemented!();
}

pub fn set_perm(p: &Path, perm: FilePermissions) -> io::Result<()> {
     unimplemented!();
}

pub fn rmdir(p: &Path) -> io::Result<()> {
     unimplemented!();
}

pub fn readlink(p: &Path) -> io::Result<PathBuf> {
     unimplemented!();
}

pub fn symlink(src: &Path, dst: &Path) -> io::Result<()> {
     unimplemented!();
}

pub fn link(src: &Path, dst: &Path) -> io::Result<()> {
     unimplemented!();
}

pub fn stat(p: &Path) -> io::Result<FileAttr> {
     unimplemented!();
}

pub fn lstat(p: &Path) -> io::Result<FileAttr> {
     unimplemented!();
}

pub fn utimes(p: &Path, atime: u64, mtime: u64) -> io::Result<()> {
     unimplemented!();
}

pub fn canonicalize(p: &Path) -> io::Result<PathBuf> {
     unimplemented!();
}
