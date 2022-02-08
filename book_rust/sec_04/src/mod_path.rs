mod aaa {
    pub mod bbb {
        pub mod ccc {
            pub fn print() {
                println!("aaa::bbb::ccc::print");
            }
        } /* ccc */
    } /* bbb */
    pub mod ddd {
        pub mod eee {
            pub fn print() {
                println!("aaa::ddd::eee:print");
            }
        } /* eee */
        pub mod fff {
            pub fn print() {
                super::eee::print();
                super::super::bbb::ccc::print();
            }
        } /* fff */
    } /* ddd */
} /* aaa */

pub fn main() {
    aaa::bbb::ccc::print();
    aaa::ddd::eee::print();
    crate::aaa::ddd::fff::print();
}
