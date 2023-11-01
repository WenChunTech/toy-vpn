use std::{mem::swap, ops::Range};

#[derive(Debug)]
pub struct IpPacket {
    pub version: u8,
    pub ihl: u8,
    pub tos: u8,
    pub total_len: u16,
    pub id: u16,
    pub flags: u8,
    pub frag_offset: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub checksum: u16,
    pub src_ip: (u8, u8, u8, u8),
    pub dst_ip: (u8, u8, u8, u8),
}

impl IpPacket {
    pub fn new(buf: &[u8]) -> Self {
        IpPacket {
            version: buf[0] >> 4,
            ihl: buf[0] & 0b00001111,
            tos: buf[1],
            total_len: u16::from_be_bytes([buf[2], buf[3]]),
            id: u16::from_be_bytes([buf[4], buf[5]]),
            flags: buf[6] >> 5,
            frag_offset: u16::from_be_bytes([buf[6] & 0b00011111, buf[7]]),
            ttl: buf[8],
            protocol: buf[9],
            checksum: u16::from_be_bytes([buf[10], buf[11]]),
            src_ip: (buf[12], buf[13], buf[14], buf[15]),
            dst_ip: (buf[16], buf[17], buf[18], buf[19]),
        }
    }
}

impl Into<Vec<u8>> for IpPacket {
    fn into(self) -> Vec<u8> {
        let mut buf = vec![0u8; self.total_len as usize];
        buf[0] = self.version << 4 | self.ihl;
        buf[1] = self.tos;
        buf[2..4].copy_from_slice(&self.total_len.to_be_bytes());
        buf[4..6].copy_from_slice(&self.id.to_be_bytes());
        buf[6] = self.flags << 5 | (self.frag_offset >> 8) as u8;
        buf[7] = self.frag_offset as u8;
        buf[8] = self.ttl;
        buf[9] = self.protocol;
        buf[10..12].copy_from_slice(&self.checksum.to_be_bytes());
        buf[12..13].copy_from_slice(&self.src_ip.0.to_be_bytes());
        buf[13..14].copy_from_slice(&self.src_ip.1.to_be_bytes());
        buf[14..15].copy_from_slice(&self.src_ip.2.to_be_bytes());
        buf[15..16].copy_from_slice(&self.src_ip.3.to_be_bytes());
        buf[16..17].copy_from_slice(&self.dst_ip.0.to_be_bytes());
        buf[17..18].copy_from_slice(&self.dst_ip.1.to_be_bytes());
        buf[18..19].copy_from_slice(&self.dst_ip.2.to_be_bytes());
        buf[19..20].copy_from_slice(&self.dst_ip.3.to_be_bytes());
        buf
    }
}

#[derive(Debug)]
pub struct UdpPacket {
    pub src_port: u16,
    pub dst_port: u16,
    pub len: u16,
    pub checksum: u16,
    pub data: Vec<u8>,
}

impl UdpPacket {
    pub fn new(buf: &[u8]) -> Self {
        UdpPacket {
            src_port: u16::from_be_bytes([buf[0], buf[1]]),
            dst_port: u16::from_be_bytes([buf[2], buf[3]]),
            len: u16::from_be_bytes([buf[4], buf[5]]),
            checksum: u16::from_be_bytes([buf[6], buf[7]]),
            data: buf[8..].to_vec(),
        }
    }
}

impl Into<Vec<u8>> for UdpPacket {
    fn into(self) -> Vec<u8> {
        let mut buf = vec![0u8; self.len as usize];
        buf[0..2].copy_from_slice(&self.src_port.to_be_bytes());
        buf[2..4].copy_from_slice(&self.dst_port.to_be_bytes());
        buf[4..6].copy_from_slice(&self.len.to_be_bytes());
        buf[6..8].copy_from_slice(&self.checksum.to_be_bytes());
        buf[8..].copy_from_slice(&self.data);
        buf
    }
}

pub fn udp_echo(mut ip_packet: IpPacket, mut udp_packet: UdpPacket) -> Vec<u8> {
    // compute checksum for ip protocol
    ip_packet.checksum = 0;
    // change the ip src and dst
    swap(&mut ip_packet.src_ip, &mut ip_packet.dst_ip);
    let mut checksum = 0u32;

    let first_byte = u8::from_be_bytes([ip_packet.version << 4 | ip_packet.ihl]);

    checksum += u16::from_be_bytes([first_byte, ip_packet.tos]) as u32;
    checksum += ip_packet.total_len as u32;
    checksum += ip_packet.id as u32;
    checksum += ((ip_packet.flags as u16) << 13 | ip_packet.frag_offset) as u32;
    checksum += u16::from_be_bytes([ip_packet.ttl, ip_packet.protocol]) as u32;
    checksum += u16::from_be_bytes([ip_packet.src_ip.0, ip_packet.src_ip.1]) as u32;
    checksum += u16::from_be_bytes([ip_packet.src_ip.2, ip_packet.src_ip.3]) as u32;
    checksum += u16::from_be_bytes([ip_packet.dst_ip.0, ip_packet.dst_ip.1]) as u32;
    checksum += u16::from_be_bytes([ip_packet.dst_ip.2, ip_packet.dst_ip.3]) as u32;

    while checksum > 0xffff {
        checksum = (checksum & 0xffff) + (checksum >> 16);
    }

    ip_packet.checksum = !(checksum as u16);

    // compute checksum for udp protocol
    udp_packet.checksum = 0;
    // change udp src and dst port
    swap(&mut udp_packet.src_port, &mut udp_packet.dst_port);
    let mut checksum = 0u32;
    // RFC 768 UDP伪头部
    //  0      7 8     15 16    23 24    31
    // +--------+--------+--------+--------+
    // |          source address           |
    // +--------+--------+--------+--------+
    // |        destination address        |
    // +--------+--------+--------+--------+
    // |  zero  |protocol|   UDP length    |
    // +--------+--------+--------+--------+

    checksum += u16::from_be_bytes([ip_packet.src_ip.0, ip_packet.src_ip.1]) as u32;
    checksum += u16::from_be_bytes([ip_packet.src_ip.2, ip_packet.src_ip.3]) as u32;
    checksum += u16::from_be_bytes([ip_packet.dst_ip.0, ip_packet.dst_ip.1]) as u32;
    checksum += u16::from_be_bytes([ip_packet.dst_ip.2, ip_packet.dst_ip.3]) as u32;
    checksum += udp_packet.src_port as u32;
    checksum += udp_packet.dst_port as u32;
    checksum += udp_packet.len as u32;
    let udp_data_end = udp_packet.data.len() - 8 - 1;
    let mut index = 1;
    while index <= udp_data_end {
        checksum += u16::from_be_bytes([udp_packet.data[index - 1], udp_packet.data[index]]) as u32;
        index += 2;
    }

    // 0 1 2 3 4 5 6
    // 1 2 3 4 5 6 7

    if index - udp_data_end == 1 {
        checksum += u16::from_be_bytes([udp_packet.data[udp_data_end], 0]) as u32;
    }

    while checksum > 0xffff {
        checksum = (checksum & 0xffff) + (checksum >> 16);
    }
    udp_packet.checksum = !(checksum as u16);

    let mut resp: Vec<u8> = ip_packet.into();
    resp.append(&mut udp_packet.into());
    resp
}
