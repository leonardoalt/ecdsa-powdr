#![no_std]
#![no_main]

use verify::verify_test;
use runtime::print;

#[no_mangle]
pub extern "C" fn _start() /*-> !*/ {
    // Initialization code can go here

    // Call the actual main function
    main();

    // Infinite loop
    //loop {
    //    nop();
    //}
}


#[no_mangle]
fn main() {
    if verify_test() {
        print!("Yeeeeeeeeeeet");
    } else {
        panic!("Oh noes");
    }
}
