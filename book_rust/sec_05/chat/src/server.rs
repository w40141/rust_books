use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread,
    time::Duration,
};

pub fn main() {
    // 1
    let server_addr = "127.0.0.1:8888";
    // 2
    let (tx, rx) = mpsc::channel::<String>();
    // 3
    let mut clients: Vec<TcpStream> = Vec::new();

    // 4
    let server = TcpListener::bind(server_addr).expect("Failed to start server.");
    server.set_nonblocking(true).expect("Cannot use");
    println!("{server_addr}でサーバーを起動しました。");

    // 5
    loop {
        // 6
        if let Ok((client, addr)) = server.accept() {
            println!("クライアントが接続: {addr}");
            clients.push(client.try_clone().unwrap());
            start_thread(client, tx.clone());
        }

        // 7
        if let Ok(msg) = rx.try_recv() {
            println!("全員に送信: {}", msg.trim());
            clients = send_all(clients, &msg);
        }
        thread::sleep(Duration::from_millis(100));
    }
}

// 8
fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);
    thread::spawn(move || loop {
        // 9
        let mut msg = String::new();
        if let Ok(n) = reader.read_line(&mut msg) {
            // 10
            if n > 0 {
                tx.send(msg).unwrap();
            }
        }
        thread::sleep(Duration::from_millis(1000));
    });
}

// 11
fn send_all(clients: Vec<TcpStream>, s: &str) -> Vec<TcpStream> {
    let mut collector = vec![];
    for mut socket in clients.into_iter() {
        let bytes = String::from(s).into_bytes();
        if let Err(e) = socket.write_all(&bytes) {
            println!("送信エラー: {e}");
            continue;
        }
        collector.push(socket);
    }
    collector
}
