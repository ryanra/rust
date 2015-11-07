// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use prelude::v1::*;

use ffi::{CStr, CString};
use fmt;
use io::{self, Error, ErrorKind};
use libc::{self, c_int, c_char, c_void, socklen_t};
use mem;
use net::{SocketAddr, Shutdown, IpAddr};
use ptr;
use str::from_utf8;
use sys::net::{cvt, cvt_r, cvt_gai, Socket, init, wrlen_t};
use sys_common::{AsInner, FromInner, IntoInner};
use time::Duration;

////////////////////////////////////////////////////////////////////////////////
// sockaddr and misc bindings
////////////////////////////////////////////////////////////////////////////////

pub fn setsockopt<T>(sock: &Socket, opt: c_int, val: c_int,
                     payload: T) -> io::Result<()> {
    unimplemented!();
}

pub fn getsockopt<T: Copy>(sock: &Socket, opt: c_int,
                       val: c_int) -> io::Result<T> {
    unimplemented!();
}

fn sockname<F>(f: F) -> io::Result<SocketAddr>
    where F: FnOnce(*mut libc::sockaddr, *mut socklen_t) -> c_int
{
    unimplemented!();
}

fn sockaddr_to_addr(storage: &libc::sockaddr_storage,
                    len: usize) -> io::Result<SocketAddr> {
    match storage.ss_family as libc::c_int {
        libc::AF_INET => {
            assert!(len as usize >= mem::size_of::<libc::sockaddr_in>());
            Ok(SocketAddr::V4(FromInner::from_inner(unsafe {
                *(storage as *const _ as *const libc::sockaddr_in)
            })))
        }
        libc::AF_INET6 => {
            assert!(len as usize >= mem::size_of::<libc::sockaddr_in6>());
            Ok(SocketAddr::V6(FromInner::from_inner(unsafe {
                *(storage as *const _ as *const libc::sockaddr_in6)
            })))
        }
        _ => {
            Err(Error::new(ErrorKind::InvalidInput, "invalid argument"))
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// get_host_addresses
////////////////////////////////////////////////////////////////////////////////

extern "system" {
    fn getaddrinfo(node: *const c_char, service: *const c_char,
                   hints: *const libc::addrinfo,
                   res: *mut *mut libc::addrinfo) -> c_int;
    fn freeaddrinfo(res: *mut libc::addrinfo);
}

pub struct LookupHost {
    original: *mut libc::addrinfo,
    cur: *mut libc::addrinfo,
}

impl Iterator for LookupHost {
    type Item = io::Result<SocketAddr>;
    fn next(&mut self) -> Option<io::Result<SocketAddr>> {
        unsafe {
            if self.cur.is_null() { return None }
            let ret = sockaddr_to_addr(mem::transmute((*self.cur).ai_addr),
                                       (*self.cur).ai_addrlen as usize);
            self.cur = (*self.cur).ai_next as *mut libc::addrinfo;
            Some(ret)
        }
    }
}

unsafe impl Sync for LookupHost {}
unsafe impl Send for LookupHost {}

impl Drop for LookupHost {
    fn drop(&mut self) {
        unsafe { freeaddrinfo(self.original) }
    }
}

pub fn lookup_host(host: &str) -> io::Result<LookupHost> {
    init();

    let c_host = try!(CString::new(host));
    let mut res = ptr::null_mut();
    unsafe {
        try!(cvt_gai(getaddrinfo(c_host.as_ptr(), ptr::null(), ptr::null(),
                                 &mut res)));
        Ok(LookupHost { original: res, cur: res })
    }
}

////////////////////////////////////////////////////////////////////////////////
// lookup_addr
////////////////////////////////////////////////////////////////////////////////

extern "system" {
    fn getnameinfo(sa: *const libc::sockaddr, salen: socklen_t,
                   host: *mut c_char, hostlen: libc::size_t,
                   serv: *mut c_char, servlen: libc::size_t,
                   flags: c_int) -> c_int;
}

const NI_MAXHOST: usize = 1025;

pub fn lookup_addr(addr: &IpAddr) -> io::Result<String> {
    init();

    let saddr = SocketAddr::new(*addr, 0);
    let (inner, len) = saddr.into_inner();
    let mut hostbuf = [0 as c_char; NI_MAXHOST];

    let data = unsafe {
        try!(cvt_gai(getnameinfo(inner, len,
                                 hostbuf.as_mut_ptr(), NI_MAXHOST as libc::size_t,
                                 ptr::null_mut(), 0, 0)));

        CStr::from_ptr(hostbuf.as_ptr())
    };

    match from_utf8(data.to_bytes()) {
        Ok(name) => Ok(name.to_owned()),
        Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                     "failed to lookup address information"))
    }
}

////////////////////////////////////////////////////////////////////////////////
// TCP streams
////////////////////////////////////////////////////////////////////////////////

pub struct TcpStream;

impl TcpStream {
    pub fn connect(addr: &SocketAddr) -> io::Result<TcpStream> {
        unimplemented!();
    }

    pub fn socket(&self) -> &Socket { unimplemented!(); }

    pub fn into_socket(self) -> Socket { unimplemented!(); }

    pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        unimplemented!();
    }

    pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        unimplemented!();
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        unimplemented!();
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        unimplemented!();
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!();
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        unimplemented!();
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        unimplemented!();
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        unimplemented!();
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        unimplemented!();
    }

    pub fn duplicate(&self) -> io::Result<TcpStream> {
        unimplemented!();
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

////////////////////////////////////////////////////////////////////////////////
// TCP listeners
////////////////////////////////////////////////////////////////////////////////

pub struct TcpListener;

impl TcpListener {
    pub fn bind(addr: &SocketAddr) -> io::Result<TcpListener> {
        unimplemented!();
    }

    pub fn socket(&self) -> &Socket { unimplemented!(); }

    pub fn into_socket(self) -> Socket { unimplemented!(); }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        unimplemented!();
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        unimplemented!();
    }

    pub fn duplicate(&self) -> io::Result<TcpListener> {
        unimplemented!();
    }
}

impl FromInner<Socket> for TcpListener {
    fn from_inner(socket: Socket) -> TcpListener {
        unimplemented!();
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

////////////////////////////////////////////////////////////////////////////////
// UDP
////////////////////////////////////////////////////////////////////////////////

pub struct UdpSocket;

impl UdpSocket {
    pub fn bind(addr: &SocketAddr) -> io::Result<UdpSocket> {
        unimplemented!();
    }

    pub fn socket(&self) -> &Socket { unimplemented!(); }

    pub fn into_socket(self) -> Socket { unimplemented!(); }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        unimplemented!();
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        unimplemented!();
    }

    pub fn send_to(&self, buf: &[u8], dst: &SocketAddr) -> io::Result<usize> {
        unimplemented!();
    }

    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        unimplemented!();
    }

    pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        unimplemented!();
    }

    pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        unimplemented!();
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        unimplemented!();
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        unimplemented!();
    }
}

impl FromInner<Socket> for UdpSocket {
    fn from_inner(socket: Socket) -> UdpSocket {
        unimplemented!();
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}
