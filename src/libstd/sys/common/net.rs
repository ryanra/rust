// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use cmp;
use ffi::CString;
use fmt;
use io::{self, Error, ErrorKind};
use libc::{c_int, c_void};
use mem;
use net::{SocketAddr, Shutdown, Ipv4Addr, Ipv6Addr};
use ptr;
use sys::net::{cvt, cvt_r, cvt_gai, Socket, init, wrlen_t};
use sys::net::netc as c;
use sys_common::{AsInner, FromInner, IntoInner};
use time::Duration;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd",
          target_os = "ios", target_os = "macos",
          target_os = "openbsd", target_os = "netbsd",
          target_os = "solaris", target_os = "haiku"))]
use sys::net::netc::IPV6_JOIN_GROUP as IPV6_ADD_MEMBERSHIP;
#[cfg(not(any(target_os = "dragonfly", target_os = "freebsd",
              target_os = "ios", target_os = "macos",
              target_os = "openbsd", target_os = "netbsd",
              target_os = "solaris", target_os = "haiku")))]
use sys::net::netc::IPV6_ADD_MEMBERSHIP;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd",
          target_os = "ios", target_os = "macos",
          target_os = "openbsd", target_os = "netbsd",
          target_os = "solaris", target_os = "haiku"))]
use sys::net::netc::IPV6_LEAVE_GROUP as IPV6_DROP_MEMBERSHIP;
#[cfg(not(any(target_os = "dragonfly", target_os = "freebsd",
              target_os = "ios", target_os = "macos",
              target_os = "openbsd", target_os = "netbsd",
              target_os = "solaris", target_os = "haiku")))]
use sys::net::netc::IPV6_DROP_MEMBERSHIP;

#[cfg(any(target_os = "linux", target_os = "android",
          target_os = "dragonfly", target_os = "freebsd",
          target_os = "openbsd", target_os = "netbsd",
          target_os = "haiku", target_os = "bitrig"))]
use libc::MSG_NOSIGNAL;
#[cfg(not(any(target_os = "linux", target_os = "android",
              target_os = "dragonfly", target_os = "freebsd",
              target_os = "openbsd", target_os = "netbsd",
              target_os = "haiku", target_os = "bitrig")))]
const MSG_NOSIGNAL: c_int = 0x0;

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
    where F: FnOnce(*mut c::sockaddr, *mut c::socklen_t) -> c_int
{
    unimplemented!();
}

fn sockaddr_to_addr(storage: &c::sockaddr_storage,
                    len: usize) -> io::Result<SocketAddr> {
    unimplemented!();
}

#[cfg(target_os = "android")]
fn to_ipv6mr_interface(value: u32) -> c_int {
    value as c_int
}

#[cfg(not(target_os = "android"))]
fn to_ipv6mr_interface(value: u32) -> ::libc::c_uint {
    value as ::libc::c_uint
}

////////////////////////////////////////////////////////////////////////////////
// get_host_addresses
////////////////////////////////////////////////////////////////////////////////

pub struct LookupHost {
    original: *mut c::addrinfo,
    cur: *mut c::addrinfo,
}

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        loop {
            unsafe {
                let cur = match self.cur.as_ref() {
                    None => return None,
                    Some(c) => c,
                };
                self.cur = cur.ai_next;
                match sockaddr_to_addr(mem::transmute(cur.ai_addr),
                                       cur.ai_addrlen as usize)
                {
                    Ok(addr) => return Some(addr),
                    Err(_) => continue,
                }
            }
        }
    }
}

unsafe impl Sync for LookupHost {}
unsafe impl Send for LookupHost {}

impl Drop for LookupHost {
    fn drop(&mut self) {
        unimplemented!();
    }
}

pub fn lookup_host(host: &str) -> io::Result<LookupHost> {
    unimplemented!();
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

    pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
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

    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        unimplemented!();
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        unimplemented!();
    }

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        unimplemented!();
    }

    pub fn ttl(&self) -> io::Result<u32> {
        unimplemented!();
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        unimplemented!();
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        unimplemented!();
    }
}

impl FromInner<Socket> for TcpStream {
    fn from_inner(socket: Socket) -> TcpStream {
        TcpStream { inner: socket }
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
        sockname(|buf, len| unsafe {
            c::getsockname(*self.inner.as_inner(), buf, len)
        })
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        unimplemented!();
    }

    pub fn duplicate(&self) -> io::Result<TcpListener> {
        unimplemented!();
    }

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        setsockopt(&self.inner, c::IPPROTO_IP, c::IP_TTL, ttl as c_int)
    }

    pub fn ttl(&self) -> io::Result<u32> {
        let raw: c_int = getsockopt(&self.inner, c::IPPROTO_IP, c::IP_TTL)?;
        Ok(raw as u32)
    }

    pub fn set_only_v6(&self, only_v6: bool) -> io::Result<()> {
        setsockopt(&self.inner, c::IPPROTO_IPV6, c::IPV6_V6ONLY, only_v6 as c_int)
    }

    pub fn only_v6(&self) -> io::Result<bool> {
        let raw: c_int = getsockopt(&self.inner, c::IPPROTO_IPV6, c::IPV6_V6ONLY)?;
        Ok(raw != 0)
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.inner.take_error()
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.inner.set_nonblocking(nonblocking)
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

    pub fn set_broadcast(&self, broadcast: bool) -> io::Result<()> {
        unimplemented!();
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        unimplemented!();
    }

    pub fn set_multicast_loop_v4(&self, multicast_loop_v4: bool) -> io::Result<()> {
        unimplemented!();
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        unimplemented!();
    }

    pub fn set_multicast_ttl_v4(&self, multicast_ttl_v4: u32) -> io::Result<()> {
        unimplemented!();
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        unimplemented!();
    }

    pub fn set_multicast_loop_v6(&self, multicast_loop_v6: bool) -> io::Result<()> {
        unimplemented!();
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        unimplemented!();
    }

    pub fn join_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr)
                         -> io::Result<()> {
        unimplemented!();
    }

    pub fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32)
                         -> io::Result<()> {
        unimplemented!();
    }

    pub fn leave_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr)
                          -> io::Result<()> {
        unimplemented!();
    }

    pub fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32)
                          -> io::Result<()> {
        unimplemented!();
    }

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        unimplemented!();
    }

    pub fn ttl(&self) -> io::Result<u32> {
        unimplemented!();
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        unimplemented!();
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        unimplemented!();
    }

    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!();
    }

    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        unimplemented!();
    }

    pub fn connect(&self, addr: &SocketAddr) -> io::Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use collections::HashMap;

    #[test]
    fn no_lookup_host_duplicates() {
        let mut addrs = HashMap::new();
        let lh = match lookup_host("localhost") {
            Ok(lh) => lh,
            Err(e) => panic!("couldn't resolve `localhost': {}", e)
        };
        let _na = lh.map(|sa| *addrs.entry(sa).or_insert(0) += 1).count();
        assert!(addrs.values().filter(|&&v| v > 1).count() == 0);
    }
}
