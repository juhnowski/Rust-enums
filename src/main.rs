enum IPAddrType {
        V4(String),
        V6(String),
}

fn main() {
    let four = IPAddrType::V4;
    let six = IPAddrType::V6;

    let home = IPAddrType::V4(String::from("127.0.0.1"));
    let loopback = IPAddrType::V6(String::from("::1"));
}
