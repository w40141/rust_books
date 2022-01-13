mod emulation_cpu;
mod make_random;

use std::intrinsics::transmute;

fn main() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);

    let a: f32 = 42.42;
    let frankentype: u32 = unsafe { transmute(a) };

    println!("{}", frankentype);
    println!("{:032b}", frankentype);
    let b: f32 = unsafe { transmute(frankentype) };
    println!("{}", b);
    assert_eq!(a, b);

    let big_endian: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
    let little_endian: [u8; 4] = [0xdd, 0xcc, 0xbb, 0xaa];

    let a: i32 = unsafe { transmute(big_endian) };
    let b: i32 = unsafe { transmute(little_endian) };

    println!("{} vs {}", a, b)
}
