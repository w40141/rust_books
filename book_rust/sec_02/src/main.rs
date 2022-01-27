use std::{
    collections::HashMap,
    env,
    fs::{read_to_string, File},
    io::{BufRead, BufReader, BufWriter, Write},
    path,
};

use num_bigint::BigInt;
use rand::{prelude::SliceRandom, Rng};

fn big_int_pow(i: i32, j: u32) -> BigInt {
    let v = BigInt::from(i);
    v.pow(j)
}

#[test]
fn big_int_pow_test() {
    let actual = big_int_pow(2, 3);
    let expect = BigInt::from(8);
    assert_eq!(actual, expect);
}

fn dice(n: i32, count: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let dices = vec![rng.gen_range(1..=n); count as usize];
    dices
}

#[test]
fn dice_test() {
    let actual = dice(6, 10);
    let expect = 10;
    assert_eq!(actual.len(), expect);
}

fn maze(map_n: usize) {
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

fn calc_bmi(height_cm: f64, weight: f64) -> f64 {
    let height = height_cm / 100.0;
    weight / height.powf(2.0)
}

fn bmi() {
    let height_cm = input("身長(cm)は？");
    let weight = input("体重(kg)は？");
    let bmi = calc_bmi(height_cm, weight);
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

fn questions() {
    const V_DATA: &str = "C, C, A, A, A, B, C, C, B, B, B, C, B, C, B, A, C, C, B, C, C, C";
    let mut c_map = HashMap::new();
    c_map.insert("A", 0);
    c_map.insert("B", 0);
    c_map.insert("C", 0);
    for v in V_DATA.split(", ") {
        c_map.insert(v, c_map[v] + 1);
    }
    for k in ["A", "B", "C"] {
        println!("{}: {:>2}", k, c_map[k]);
    }
}

fn months() {
    let tuki = [
        "睦月",
        "如月",
        "弥生",
        "卯月",
        "皐月",
        "水無月",
        "文月",
        "葉月",
        "長月",
        "神無月",
        "霜月",
        "師走",
    ];

    let mut t_map: HashMap<&str, usize> = HashMap::new();
    for (i, j) in tuki.iter().enumerate() {
        t_map.insert(j, i + 1);
    }
    println!("水無月 = {}月", t_map["水無月"]);
    println!("文月 = {}月", t_map["文月"]);
    println!("師走 = {}月", t_map["師走"]);
}

fn not_exist() {
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);
    map.insert("D", 100);
    match map.get("D") {
        Some(v) => println!("{v}"),
        _ => println!("not exist"),
    }
}

fn sum_args() {
    let args: Vec<String> = env::args().collect();
    let total = sum_from_args(args);
    println!("{total}");
}

fn sum_from_args(args: Vec<String>) -> f64 {
    let mut total = 0.0;
    for (i, s) in args.iter().enumerate() {
        if i == 0 {
            continue;
        } else {
            let num: f64 = match s.parse() {
                Ok(v) => v,
                Err(_) => 0.0,
            };
            total += num;
        }
    }
    total
}

fn read_file() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Input file name");
        return;
    } else {
        let filename = &args[1];
        let text = read_to_string(filename).unwrap();
        println!("{text}");
    }
}

fn sum_from_read_file() {
    let args = env::args();
    let mut total: f64 = 0.0;
    for (i, fname) in args.enumerate() {
        if i == 0 {
            continue;
        }
        let text = read_to_string(fname).unwrap();
        let lines = text.split('\n');
        for line in lines {
            let n: f64 = match line.parse() {
                Ok(v) => v,
                Err(_) => 0.0,
            };
            total += n;
        }
    }
    println!("{total}");
}

fn cli_dict() {
    let dictionary = "../../../../../../../Downloads/ejdic-hand-txt/ejdict-hand-utf8.txt";
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("[USAGE] jisyo word");
        return;
    }

    let word = &args[1];
    let fp = File::open(dictionary).unwrap();
    let reader = BufReader::new(fp);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.find(word) == None {
            continue;
        }
        println!("{line}")
    }
}

fn write_fizzbuzz() {
    let fname = "./src/fizzbuzz.txt";
    {
        let fp = File::create(fname).unwrap();
        let mut write = BufWriter::new(fp);
        let fb = fizzbuzz(1, 101);
        for v in fb {
            let line = v + "\n";
            let bytes = line.as_bytes();
            write.write(bytes).unwrap();
        }
    }
    let s = read_to_string(fname).unwrap();
    println!("{s}")
}

fn fb(i: i32) -> String {
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
    s
}

fn fizzbuzz(start: i32, end: i32) -> Vec<String> {
    let v: Vec<String> = (start..end).map(|x| fb(x)).collect();
    v
}

fn sum(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else {
        return sum(n - 1) + n;
    }
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

fn find() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("find file (path) (keyword)");
        return;
    }
    let target_dir = &args[1];
    let keyword = &args[2];
    let target = path::PathBuf::from(target_dir);
    findfile(&target, keyword);
}

fn findfile(target: &path::PathBuf, keyword: &str) {
    let files = target.read_dir().expect("not exist path");
    for dir_entry in files {
        let path = dir_entry.unwrap().path();
        if path.is_dir() {
            findfile(&path, keyword);
            continue;
        }
        let fname = path.file_name().unwrap().to_string_lossy();
        if fname.find(keyword) == None {
            continue;
        }
        println!("{}", path.to_string_lossy());
    }
}

fn tree() {
    let args: Vec<String> = env::args().collect();
    let mut target_dir = ".";
    if args.len() >= 2 {
        target_dir = &args[1]
    }
    let target = path::PathBuf::from(target_dir);
    println!("{target_dir}");
    treefile(&target, 0);
}

fn treefile(target: &path::PathBuf, level: isize) {
    let files = target.read_dir().expect("not exist");
    for ent in files {
        let path = ent.unwrap().path();
        for _ in 1..=level {
            print!("|    ");
        }
        let fname = path.file_name().unwrap().to_string_lossy();
        if path.is_dir() {
            println!("|-- <{}>", fname);
            treefile(&path, level + 1);
            continue;
        }
        println!("|-- {fname}");
    }
}

fn main() {
    // dice();
    // maze();
    // bingo();
    // bmi();
    // questions();
    // months();
    // not_exist();
    // cli_dict();
    // write_fizzbuzz();
    // find();
    tree();
}
