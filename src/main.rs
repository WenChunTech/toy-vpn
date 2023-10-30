#![allow(unused)]
mod ffi;
mod protocol;
mod tun;
use std::{ffi::CString, io::Read, os::fd::FromRawFd};

use protocol::{IpPacket, UdpPacket};
use tun::{set_tun_ip, set_tun_route};

fn main() {
    // let dev_name = "tun0" as *const str as *const c_char;
    let dev_name = CString::new("tun0").expect("Setting Dev Name Failure!!!");
    let c_str = dev_name.as_ptr();
    let fd = unsafe { ffi::tun_create(c_str) };
    if fd == -1 {
        panic!("failed to create tun device");
    }
    set_tun_ip(dev_name.to_str().unwrap_or("tun0"), "10.1.1.2/24");
    set_tun_route(dev_name.to_str().unwrap_or("tun0"), "10.1.1.0/24");
    let mut file = unsafe { std::fs::File::from_raw_fd(fd) };

    loop {
        let mut buf = [0u8; 1024];
        let n = file.read(&mut buf).unwrap();
        let ip_packet = IpPacket::new(&buf[..n]);
        println!("{ip_packet:#?}");
        let udp_packet_start = ip_packet.ihl as usize * 4;
        let udp_packet_end = ip_packet.total_len as usize;
        let ip_packet_data = &buf[udp_packet_start..udp_packet_end];
        let udp_pack = UdpPacket::new(ip_packet_data);
        println!("{udp_pack:#?}");
    }
}
