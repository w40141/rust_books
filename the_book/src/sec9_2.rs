use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

pub fn main() {
    // let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
    //         Ok(fc) => fc,
    //         Err(e) => {
    //             panic!("Tried to create file but there was a problem: {:?}", e)
    //         }
    //     },
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error);
    //     },
    // };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)

    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // 
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
}
