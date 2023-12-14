enum IpAddressKinds {
    ipv4
    ipv6
}

enum IpAddressKinds2 {
    V4(String)
    V6(String)
}

enum IpAddressKinds3 {
    V4(u8, u8, u8, u8)
    V6(String)
}

struct IpAddresses {
    kind: IpAddressKinds;
    address: String;
}

fn main() {
    let home = IpAddresses {
        kind: IpAddresses::ipv4,
        address: String::from("127.0.0.1"),
    }

    let proxy = IpAddresses {
        kind: IpAddresses::ipv6,
        address: String::from("::1")
    }

    let home2 = IpAddressKinds2::V4(String::from("127.0.0.1"));
    let proxy2 = IpAddressKinds2::V6(String::from("::1"));    
    
    let home2 = IpAddressKinds3::V4(String::from(127, 0, 0, 1));
    let proxy2 = IpAddressKinds3::V6(String::from("::1"));
}

fn route(ip_kind: IpAddressKinds) {}