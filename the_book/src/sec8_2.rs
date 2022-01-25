pub fn main() {
    // let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();
    // let s = String::from("initial contents");

    let mut t = String::from("foo");
    t.push_str(" bar");
    println!("{}", t);

    let s1 = String::from("hello");
    let len = String::from("Hola").len();
    println!("{}", len);
    let len = String::from("Здравствуйте").len();
    println!("{}", len);
}
