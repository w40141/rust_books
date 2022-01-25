// enum IpAddrKind {
//     v4,
//     v6,
// }
//
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum IpAddr {
    v4(u8, u8, u8, u8),
    v6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //
    }
}

fn route(ip_type: IpAddrKind) {}

pub fn main() {
    // let home = IpAddr {
    //     kind: IpAddrKind::v4,
    //     address: String::from("127.0.0.1"),
    // };
    //
    // let loopback = IpAddr {
    //     kind: IpAddrKind::v6,
    //     address: String::from("::1"),
    // };
    // let home = IpAddr::v4(String::from("127.0.0.1"));
    let home = IpAddr::v4(String::from(127, 0, 0, 1));
    let loopback = IpAddr::v6(String::from("::1"));
    let m = Message::Write(String::from("hello"));
    m.call();
}
