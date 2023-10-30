use std::ops::Range;

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

#[derive(Debug)]
pub struct UdpPacket {
    pub src_port: u16,
    pub dst_port: u16,
    pub len: u16,
    pub checksum: u16,
}

impl UdpPacket {
    pub fn new(buf: &[u8]) -> Self {
        UdpPacket {
            src_port: u16::from_be_bytes([buf[0], buf[1]]),
            dst_port: u16::from_be_bytes([buf[2], buf[3]]),
            len: u16::from_be_bytes([buf[4], buf[5]]),
            checksum: u16::from_be_bytes([buf[6], buf[7]]),
        }
    }
}

fn udp_echo(ip_packet: IpPacket, udp_packet: UdpPacket) {
    // 0000 0100 -> 0100 0000 | 5 = 0100 0101
}
