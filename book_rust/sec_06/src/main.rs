use sec_1::sec_01;
use sec_2::sec_02;
use sec_3::sec_03;
use std::fs;

mod r#macro;
pub mod node;
pub mod parser;
pub mod runner;
pub mod sec_1;
pub mod sec_2;
pub mod sec_3;
pub mod sec_4;

fn main() {
    // sec_1::sec_01();
    // sec_2::sec_02();
    // sec_03()
    // println!("{}", calc::eval("2+5").unwrap());
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[USAGE] peg_tomato file.tomato");
        return;
    }
    let filename = &args[1];
    let src = fs::read_to_string(filename).unwrap();
    runner::run(&src);
}

#[cfg(test)]
mod tests {
    use crate::runner::run;

    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("print 32"), 32);
        assert_eq!(run("print 1+2*3"), 7);
        assert_eq!(run("print 10%3"), 1);
        assert_eq!(run("a=3; if a==3 { print 1 } else { print 0 }"), 1);
        assert_eq!(run("a=0; for i=1 to 10 { a=a+i }; print a"), 55);
        assert_eq!(run("print \"abc\""), 0);
    }
}
