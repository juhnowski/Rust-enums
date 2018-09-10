enum IPAddrType {
        V4(String),
        V6(String),
}

enum Message {
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        println!("I'm inside call");
    }
}

fn main() {
    let four = IPAddrType::V4;
    let six = IPAddrType::V6;

    let home = IPAddrType::V4(String::from("127.0.0.1"));
    let loopback = IPAddrType::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}
