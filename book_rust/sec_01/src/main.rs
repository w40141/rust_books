fn moon() {
    let distance = 384_400.0;
    let car_speed = 80.0;
    let bullet_speed = 300.0;
    let day_car = distance / car_speed / 24.0;
    let bullet_speed = distance / bullet_speed / 24.0;
    println!("{day_car}");
    println!("{bullet_speed}");
}

fn fizzbuzz() {
    for i in 1..101 {
        let mut s = String::from("");
        if i % 3 == 0 {
            s += "Fizz";
        }
        if i % 5 == 0 {
            s += "Buzz";
        }

        if s == "" {
            s += &i.to_string();
        }
        println!("{s}");
    }
}

fn nabeatu() {
    for i in 1..51 {
        if i % 3 == 0 || i % 10 == 3 {
            println!("A");
        } else if i > 30 && i < 39 {
            println!("A");
        } else {
            println!("{i}");
        }
    }
}

fn kuku() {
    for i in 1..=9 {
        let s = (1..=9)
            .map(|x| format!("{:3}", x * i))
            .collect::<Vec<String>>()
            .join(",");
        println!("{s}")
        // for j in 1..=9 {
        //     print!("{:3},", i * j);
        // }
        // println!();
    }
}

fn fibna() {
    let mut a = 0;
    let mut b = 1;
    for _ in 1..=30 {
        let n = a + b;
        println!("{n}");
        a = b;
        b = n;
    }
}

fn compare() {
    let pc = 98000.0;
    let a = pc * 0.8 + 1200.0;
    let b = pc * 0.9;
    println!("{a}, {b}")
}

fn coin() {
    let price = 3950;
    for i in 0..=10 {
        for j in 0..=3 {
            for l in 0..=10 {
                let tmp = 500 * i + 100 * j + 50 * l;
                if price == tmp {
                    println!("500円 {i}, 100円 {j}, 50円 {l}")
                }
            }
        }
    }
}

fn caesar_encode(text: &str, shift: i16) {
    let code_a = 'a' as i16;
    let code_z = 'z' as i16;
    let mut result = String::new();
    for t in text.chars() {
        let mut code = t as i16;
        if code >= code_a && code <= code_z {
            code = (code - code_a + shift) % 26 + code_a;
        }
        result.push((code as u8) as char);
    }
    println!("{result}");
}

fn caesar_decode(text: &str, shift: i16) {
    let code_a = 'a' as i16;
    let code_z = 'z' as i16;
    let mut result = String::new();
    for t in text.chars() {
        let mut code = t as i16;
        if code >= code_a && code <= code_z {
            code = (code - code_a - shift) % 26 + code_a;
        }
        result.push((code as u8) as char);
    }
    println!("{result}");
}

fn is_prime(n: usize) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn get_prime(primes: &mut [usize; 100]) {
    let mut i = 2;
    let mut count = 0;
    while count < 100 {
        if is_prime(i) {
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
}

fn main() {
    // moon();
    // fizzbuzz();
    // nabeatu();
    // kuku();
    // fibna();
    // compare();
    // coin();
    // caesar_encode("i am tom", 3);
    // caesar_decode("l dp wrp", 3)
    let mut primes = [0; 100];
    get_prime(&mut primes);
    println!("{:?}", primes);
}
