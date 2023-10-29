use std::{
    net::{Ipv4Addr, UdpSocket},
    str::FromStr,
};

pub fn create_tun(tun_name: &str) {
    let cmd = format!("ip tuntap add dev {tun_name} mode tun");
    std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to create tun device");
}

pub fn set_tun_ip(tun_name: &str, ip: &str) {
    let cmd = format!(
        "ip addr add {ip} dev {tun_name}",
        ip = ip,
        tun_name = tun_name
    );
    std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to set tun ip");

    let cmd = format!("ip link set {tun_name} up");
    std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to set tun device up");
}

pub fn set_tun_route(tun_name: &str, route: &str) {
    let cmd = format!("ip route add {route} dev {tun_name}",);
    std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to set tun route");
}

// setting os forward for ip packet
pub fn set_traffic_forward() {
    let cmd = "sysctl -w net.ipv4.ip_forward=1";
    std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to set os forward");
}

// setting ip addr masquerade
pub fn set_ip_masquerade(tun_name: &str) {
    let cmd = format!(
        "iptables -t nat -A POSTROUTING -o {tun_name} -j MASQUERADE",
        tun_name = tun_name
    );
    std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to set ip addr masqurade");
}

// setting default route to tun
pub fn set_default_route_to_tun(tun_name: &str) {
    let cmd = format!("ip route add default dev {tun_name}");
    std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to set default route to tun");
}

// 私有网段,有A,B,C三个地址段：
// 10.0.0.0/8:10.0.0.0-10.255.255.255
// 172.16.0.0/12:172.16.0.0-172.31.255.255
// 192.168.0.0/16:192.168.0.0-192.168.255.255
pub fn udp_serve() {
    let ip = Ipv4Addr::from_str("192.168.1.100").unwrap();
    let serve = UdpSocket::bind((ip, 2345)).unwrap();
    loop {
        let mut buf = [0u8; 1024];
        let size = serve.recv(&mut buf).unwrap();
        println!("recv size: {:?}", &buf[..size]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tun() {
        let tun_name = "tun0";
        create_tun(tun_name);
        set_tun_ip(tun_name, "192.168.1.100");
        // set_traffic_forward();
        set_ip_masquerade(tun_name);
        udp_serve();
        // set_default_route_to_tun(tun_name);
    }
}
