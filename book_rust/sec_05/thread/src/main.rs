use std::{sync, thread, time};

fn sleep_print(name: &str) {
    for i in 1..=3 {
        println!("{name}: i={i}");
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn sleep_sender(name: &str, sender: sync::mpsc::Sender<String>) {
    for i in 1..=5 {
        let msg = format!("{name}: {i}");
        sender.send(msg).unwrap();
        thread::sleep(time::Duration::from_millis(1000));
    }
    sender.send("quit".to_string()).unwrap();
}

fn fib(n: i64) -> i64 {
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    return fib(n - 2) + fib(n - 1);
}

fn show_time(star_time: time::Instant) {
    let elapsed = star_time.elapsed();
    println!("実行時間：{elapsed:?}")
}

pub fn main() {
    // println!("---スレッドなし---");
    // sleep_print("スレッドなし");
    //
    // println!("---スレッドあり---");
    // thread::spawn(|| sleep_print("二郎"));
    // thread::spawn(|| sleep_print("三郎"));
    // sleep_print("太郎");
    // let (tx, rx) = sync::mpsc::channel::<String>();
    // let sender = tx.clone();
    // thread::spawn(|| sleep_sender("たろう", sender));
    // let sender = tx.clone();
    // thread::spawn(|| sleep_sender("じろう", sender));
    // loop {
    //     let buf = rx.recv().unwrap();
    //     println!("[受信] {buf}");
    //     if buf == "quit" {
    //         break;
    //     }
    // }
    let request_nums = [43, 42, 20, 39, 37, 35, 30];
    let start_time = time::Instant::now();
    let (tx, rx) = sync::mpsc::channel::<(i64, i64)>();
    for num in request_nums {
        let sender = tx.clone();
        thread::spawn(move || {
            let answer = fib(num);
            sender.send((num, answer)).unwrap();
        });
        // let answer = fib(num);
        // println!("[結果] fib({num}) = {answer}");
    }
    let mut job = request_nums.len();
    loop {
        if let Ok((arg, answer)) = rx.recv() {
            job -= 1;
            println!("[結果] fib({arg}) = {answer} (残り={job})");
            if job <= 0 {
                show_time(start_time);
                break;
            }
        }
        thread::sleep(time::Duration::from_micros(300));
    }
    // show_time(star_time);
}
