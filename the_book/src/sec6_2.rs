enum Coin {
    Penny,
    Niclel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> uew {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Niclel => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
            println!("State quarter from {:?}!", state);
            35
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
