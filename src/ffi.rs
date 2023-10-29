use std::{ffi::c_char, os::fd};

extern "C" {
    pub fn tun_create(dev_name: *const c_char) -> fd::RawFd;
}
