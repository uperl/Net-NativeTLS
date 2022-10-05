extern crate native_tls;

use native_tls::TlsConnector;
use native_tls::TlsStream;
use std::io::{Read, Write};
use std::net::TcpStream;

use std::ffi::c_void;
use std::ffi::CStr;

struct NetNativeTLS<S> {
  s: TlsStream<S>,
}

impl NetNativeTLS<TcpStream> {
  fn new(hostport: &str, host: &str) -> NetNativeTLS<TcpStream> {
    let connector = TlsConnector::new().unwrap();
    let stream = TcpStream::connect(hostport).unwrap();
    let stream = connector.connect(host, stream).unwrap();
    NetNativeTLS {
      s: stream,
    }
  }
}

type CNetNativeTLS = c_void;

#[no_mangle]
pub extern "C" fn netnativetls_new(hostport: *const i8, host: *const i8) -> *mut CNetNativeTLS {
  let host = unsafe { CStr::from_ptr(host) };
  let hostport = unsafe { CStr::from_ptr(hostport) };

  let nnt = NetNativeTLS::new(hostport.to_str().unwrap(), host.to_str().unwrap());
  Box::into_raw(Box::new(nnt)) as *mut CNetNativeTLS
}

#[no_mangle]
pub extern "C" fn netnativetls_write_all(nnt: *mut CNetNativeTLS, data: *const i8) {
  let nnt  = unsafe { &mut*(nnt as *mut NetNativeTLS<TcpStream>) };
  let data = unsafe { CStr::from_ptr(data) };
  nnt.s.write_all(data.to_bytes());
}

#[no_mangle]
pub extern "C" fn netnativetls_read_to_end(nnt: *mut CNetNativeTLS) -> *const u8 {
  let mut res = vec![];
  let nnt  = unsafe { &mut*(nnt as *mut NetNativeTLS<TcpStream>) };
  nnt.s.read_to_end(&mut res).unwrap();
  let res = String::from_utf8_lossy(&res);
  res.as_ptr()
}
