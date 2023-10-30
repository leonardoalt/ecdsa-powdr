#![no_std]
#![no_main]

use verify::verify_test_verify;
use runtime::print;

#[no_mangle]
fn main() {
    if verify_test_verify() {
        print!("Yeeeeeeeeeeet");
    } else {
        panic!("Oh noes");
    }
}
