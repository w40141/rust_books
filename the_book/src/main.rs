use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || 100 < value {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn tmp(n: i32) -> Vec<i32> {
    (1..=n).collect::<Vec<i32>>()
}

#[test]
fn tmp_test() {
    let actual = tmp(5);
    let expect = vec![1, 2, 3, 4, 5];
    assert_eq!(actual, expect);
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            // Ok(num) => Guess::new(num),
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
