use crate::random::{linear, xorshift};

// pub mod mod_random;
mod mod_path;
mod random;

fn main() {
    let mut seed = 1u32;
    let r1 = linear::rand(&mut seed);
    let r2 = xorshift::rand(&mut seed);
    println!("{r1}, {r2}");

    let mut a = Counter::new();
    count(Some(&mut a));
    count(Some(&mut a));
    let a = None;
    count(a);

    let wallet = vec![
        Coin::Coin1(3),
        Coin::Coin5(10),
        Coin::Coin10(5),
        Coin::Coin50(1),
        Coin::Coin100(3),
        Coin::Coin500(2),
    ];
    let total = wallet.iter().fold(0, |sum, v| sum + v.calc_price());
    println!("{total}")
    // let box1 = JewelyBox {
    //     price: 30,
    //     key_no: 1,
    // };
    // let box2 = TrapBox { damage: 10 };
    // let box3 = JewelyBox {
    //     price: 20,
    //     key_no: 2,
    // };
    //
    // let key_no = 2;
    // open_box(&box1, key_no);
    // open_box(&box2, key_no);
    // open_box(&box3, key_no);
    //
    // let pt_i = Point { x: 20, y: 50 };
    // println!("{pt_i:?}");
    //
    // let array = [
    //     "Apple".to_string(),
    //     "Banana".to_string(),
    //     "Mango".to_string(),
    //     "Tomato".to_string(),
    // ];
    // for a in array.iter() {
    //     println!("{a}")
    // }
    // println!("len={}", array.len());
    //
    // let prime_iter = PrimeIter::new();
    // for n in prime_iter {
    //     print!("{}, ", n)
    // }
    //
    // let fib_iter = FibIter::new();
    // for (i, n) in fib_iter.enumerate() {
    //     if i >= 10 {
    //         break;
    //     }
    //     print!("{n}, ");
    // }
    // println!("");
    //
    // let fib_iter = FibIter::new();
    // fib_iter.take(20).for_each(|f| print!("{f}, "))
}

fn fizz_buzz(i: i32) {
    for n in 1..=i {
        let msg = match (n % 3, n % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => n.to_string(),
        };
        println!("{msg}");
    }
}

struct Counter {
    value: i64,
}

impl Counter {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    fn inc(&mut self) {
        self.value += 1;
        println!("value={}", self.value);
    }
}

fn count(counter: Option<&mut Counter>) {
    match counter {
        Some(c) => c.inc(),
        _ => return,
    };
}

struct FibIter {
    a: usize,
    b: usize,
}

impl FibIter {
    pub fn new() -> Self {
        Self { a: 1, b: 1 }
    }
}

impl Iterator for FibIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.a;
        self.a = self.b;
        self.b += tmp;
        return Some(self.a);
    }
}

struct PrimeIter {
    n: u8,
}

impl PrimeIter {
    fn new() -> Self {
        Self { n: 1 }
    }

    fn is_prime(&self) -> bool {
        for i in 2..self.n {
            if self.n % i == 0 {
                return false;
            }
        }
        return true;
    }
}

impl Iterator for PrimeIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n += 1;
            if std::u8::MAX == self.n {
                return None;
            }
            if self.is_prime() {
                return Some(self.n);
            }
        }
    }
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: std::ops::AddAssign,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn add(&mut self, pt: Point<T>) {
        self.x += pt.x;
        self.y += pt.y;
    }
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn add3<T>(a: T, b: T, c: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b + c
}

fn x2<T: std::ops::Add<Output = T> + Copy>(n: T) -> T {
    n + n
}

enum Coin {
    Coin1(isize),
    Coin5(isize),
    Coin10(isize),
    Coin50(isize),
    Coin100(isize),
    Coin500(isize),
}

impl Coin {
    fn calc_price(&self) -> isize {
        match *self {
            Coin::Coin1(v) => v,
            Coin::Coin5(v) => v * 5,
            Coin::Coin10(v) => v * 10,
            Coin::Coin50(v) => v * 50,
            Coin::Coin100(v) => v * 100,
            Coin::Coin500(v) => v * 500,
        }
    }
}

#[derive(Debug, PartialEq)]
enum BmiLevel {
    Lv0,
    Lv1,
    Lv2,
    Lv3,
    Lv4,
    Lv5,
}

#[derive(Debug)]
struct Body {
    height: f64,
    width: f64,
}

impl Body {
    fn new(height: f64, width: f64) -> Self {
        Self { height, width }
    }
    fn calc_bmi(&self) -> f64 {
        let h = self.height / 100.0;
        self.width / h.powf(2.0)
    }

    fn calc_per(&self) -> f64 {
        self.calc_bmi() / 22.0 * 100.0
    }

    fn calc_level(&self) -> BmiLevel {
        let bmi = self.calc_bmi();
        if bmi < 18.5 {
            BmiLevel::Lv0
        } else if bmi < 25.0 {
            BmiLevel::Lv1
        } else if bmi < 30.0 {
            BmiLevel::Lv2
        } else if bmi < 35.0 {
            BmiLevel::Lv3
        } else if bmi < 40.0 {
            BmiLevel::Lv4
        } else {
            BmiLevel::Lv5
        }
    }
}

struct Person {
    name: String,
    age: i32,
    body: Body,
}

impl Person {
    pub fn new(name: String, age: i32, height: f64, width: f64) -> Self {
        let body = Body::new(height, width);
        Self { name, age, body }
    }
}

trait TreasureBox {
    fn open(&self, key_no: i32) -> bool {
        self.get_key_no() == key_no
    }
    fn check(&self);
    fn get_key_no(&self) -> i32;
}

struct JewelyBox {
    price: i32,
    key_no: i32,
}

impl TreasureBox for JewelyBox {
    // fn open(&self, key_no: i32) -> bool {
    //     self.key_no == key_no
    // }

    fn check(&self) {
        println!("宝石箱だった！金貨{}枚を入手した。", self.price)
    }

    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

struct TrapBox {
    damage: i32,
}

impl TreasureBox for TrapBox {
    fn open(&self, _key: i32) -> bool {
        true
    }

    fn check(&self) {
        println!("罠だった！{}のダメージ。", self.damage)
    }

    fn get_key_no(&self) -> i32 {
        0
    }
}

struct EmptyBox {
    key_no: i32,
}

impl TreasureBox for EmptyBox {
    fn check(&self) {
        println!("空箱だった。")
    }
    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if !tbox.open(key_no) {
        println!("鍵が合わず宝箱が開きません。");
        return;
    }
    tbox.check();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_test() {
        let taro = Body {
            height: 160.0,
            width: 70.0,
        };
        assert_eq!(160.0, taro.height);
        assert_eq!(70.0, taro.width);
    }

    #[test]
    fn person_test() {
        let age = 10;
        let height = 160.0;
        let width = 70.0;
        let bob = Person::new("bobby".to_string(), age, height, width);
        assert_eq!("bobby", bob.name);
        assert_eq!(10, bob.age);
        assert_eq!(BmiLevel::Lv2, bob.body.calc_level());
        let jiro = Person {
            name: String::from("jiro"),
            ..bob
        };
        assert_eq!("jiro", jiro.name);
        assert_eq!(10, jiro.age);
        assert_eq!(BmiLevel::Lv2, jiro.body.calc_level());
    }
}
