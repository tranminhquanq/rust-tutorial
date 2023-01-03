enum IPAddressKind {
    V4,
    V6
}

struct QuitMessage;

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Log from message impl {:?}", self);
    }
}

struct IPAddress {
    kind: IPAddressKind,
    address: String,
}

impl IPAddress {
    fn getIPAddr(&self) -> String {
        return self.address.clone();
    }
}

fn main() {
    let four = IPAddressKind::V4;
    let six = IPAddressKind::V6;

    let home = IPAddress{
        kind: IPAddressKind::V4,
        address: String::from("127.0.0.1")
    };
    let loopback = IPAddress{
        kind: IPAddressKind::V6,
        address: String::from("::1")
    };
    println!("{}", home.getIPAddr());
    let msg = Message::Write(String::from("Hello"));
    msg.call()
}
