fn main() {
    let i = 10;
    let p = plus_one(i);
    println!("{i}, {p}");
    let q = plus_one(i);
    println!("{i}, {q}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
