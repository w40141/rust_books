use std::fmt::format;
use std::io::{stdin, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub fn main() {
    let server_addr = "127.0.0.1:8888";
    let mut socket = TcpStream::connect(server_addr).expect("Cannot connect to server.");
    socket.set_nonblocking(true).expect("Cannot use.");
    println!("{server_addr}と接続しました。");
    start_thread(socket.try_clone().unwrap());

    let user = input("What's your name?");
    println!("{user}さん、メッセージを入力してください。");

    loop {
        let msg = input("> ");
        let msg = format!("{user}> {msg}\n");
        let buf = msg.as_bytes();
        socket.write_all(buf).unwrap();
    }
}

fn start_thread(socket: TcpStream) {
    let mut reader = BufReader::new(socket);
    thread::spawn(move || loop {
        let mut buf = String::new();
        if let Ok(n) = reader.read_line(&mut buf) {
            if n > 0 {
                println!("[受信] {}", buf.trim());
            }
        }
        thread::sleep(Duration::from_millis(1000));
    });
}

fn input(msg: &str) -> String {
    if msg != "> " {
        println!("{msg}");
    }
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("標準入力エラー");
    String::from(buf.trim())
}
