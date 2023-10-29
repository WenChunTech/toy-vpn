#![allow(unused)]
mod ffi;
mod tun;
use std::{ffi::CString, io::Read, os::fd::FromRawFd};

use tun::set_tun_ip;

fn main() {
    // let dev_name = "tun0" as *const str as *const c_char;
    let dev_name = CString::new("tun0").expect("Setting Dev Name Failure!!!");
    let c_str = dev_name.as_ptr();
    let fd = unsafe { ffi::tun_create(c_str) };
    if fd == -1 {
        panic!("failed to create tun device");
    }
    set_tun_ip(dev_name.to_str().unwrap_or("tun0"), "192.168.1.10");
    let mut file = unsafe { std::fs::File::from_raw_fd(fd) };

    loop {
        let mut buf = [0u8; 1504];
        let n = file.read(&mut buf).unwrap();
        println!("read {} bytes", n);
        println!("{:?}", &buf[..n]);
    }
}
