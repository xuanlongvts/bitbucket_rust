#![allow(unused)]

fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // ============
    enum IpAddr_new {
        V4(String),
        V6(String),
    }
    let home_2 = IpAddr_new::V4(String::from("127.0.0.1"));
    let loopback_2 = IpAddr_new::V6(String::from("::1"));
}
