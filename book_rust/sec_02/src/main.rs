use num_bigint::BigInt;
use rand::{prelude::SliceRandom, Rng};

fn big_int_pow() {
    let v = BigInt::from(1234);
    println!("{}", v.pow(5678))
}

fn dice() {
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let dice = rng.gen_range(1..=6);
        println!("{dice}");
    }
}

fn maze() {
    let map_n = 25;
    let mut maze = vec![vec![0; map_n]; map_n];
    for n in 0..map_n {
        maze[0][n] = 1;
        maze[n][0] = 1;
        maze[map_n - 1][n] = 1;
        maze[n][map_n - 1] = 1;
    }

    let mut rng = rand::thread_rng();
    for y in 2..map_n - 2 {
        for x in 2..map_n - 2 {
            if x % 2 == 1 || y % 2 == 1 {
                continue;
            }
            maze[y][x] = 1;
            let r = rng.gen_range(0..=3);
            match r {
                0 => maze[y - 1][x] = 1,
                1 => maze[y + 1][x] = 1,
                2 => maze[y][x - 1] = 1,
                3 => maze[y][x + 1] = 1,
                _ => {}
            }
        }
    }
    let titles = ["  ", "ZZ"];
    for y in 0..map_n {
        for x in 0..map_n {
            print!("{}", titles[maze[y][x]]);
        }
        println!("");
    }
}

fn bingo() {
    let mut nums: Vec<i32> = (1..=75).collect();
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);
    for i in 0..5 {
        for j in 0..5 {
            let n = i * 5 + j;
            if n == 12 {
                print!("  *,");
            } else {
                print!("{:3},", nums[n]);
            }
        }
        println!("")
    }
}

fn bmi() {
    let height_cm = input("身長(cm)は？");
    let weight = input("体重(kg)は？");
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    println!("BMI={:.1}", bmi);
    if bmi < 18.5 {
        println!("低体重");
    } else if bmi < 25.0 {
        println!("普通体重");
    } else if bmi < 30.0 {
        println!("肥満度1");
    } else if bmi < 35.0 {
        println!("肥満度2");
    } else if bmi < 40.0 {
        println!("肥満度3");
    } else {
        println!("肥満度4");
    }
}

fn input(prompt: &str) -> f64 {
    println!("{prompt}");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("input error");
    return s.trim().parse().expect("transition error");
}

fn main() {
    // dice();
    // maze();
    // bingo();
    bmi();
}
