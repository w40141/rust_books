mod random {
    pub mod linear {
        use std::num::Wrapping;

        pub fn rand(seed: &mut u32) -> u32 {
            let (a, c) = (134775813u32, 12345u32);
            *seed = (Wrapping(*seed) * Wrapping(a) + Wrapping(c)).0;
            *seed
        }

        pub fn one() -> u32 {
            1
        }
    }

    pub mod xor_shift {
        pub fn rand(seed: &mut u32) -> u32 {
            let mut y = *seed;
            y ^= y << 13;
            y ^= y >> 17;
            y ^= y << 5;
            *seed = y;
            y
        }

        pub fn one() -> u32 {
            super::linear::one()
        }
    } /* xor_shift */
} /* random */

use random::{linear, xor_shift};
pub fn main() {
    let mut seed1 = 12345u32;
    let mut seed2 = 12345u32;
    for i in 0..10 {
        let r1 = linear::rand(&mut seed1) % 6 + 1;
        let r2 = xor_shift::rand(&mut seed2) % 6 + 1;
        println!("L: {i}, {r1}  {r2}");
    }
    println!("{}", xor_shift::one());
}
