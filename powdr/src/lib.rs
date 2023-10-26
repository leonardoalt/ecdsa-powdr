#![no_std]

use verify::verify_test;
use runtime::print;

#[no_mangle]
fn main() {
    if verify_test() {
        print!("Yeeeeeeeeeeet");
    } else {
        panic!("Oh noes");
    }
}
