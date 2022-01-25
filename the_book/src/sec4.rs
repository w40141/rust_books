pub fn main() {
    let n = 10;
    println!("{}", fib(n));
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn fib(n: i32) -> i32 {
    let m = if n < 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    };
    m
}
