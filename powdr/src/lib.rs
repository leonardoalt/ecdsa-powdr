#![no_std]
#![no_main]

extern crate runtime;
use verify::verify_test_verify;

#[no_mangle]
pub fn main() {
    if !verify_test_verify() {
        panic!("Oh noes");
    }
}
