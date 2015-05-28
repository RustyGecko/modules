#![no_std]
#![no_main]
#![feature(core, no_std, collections)]

#[macro_use] extern crate collections;
extern crate core;

extern crate emlib;         // Include emlib bindings
extern crate modules;       // Include `REM'
use modules::Usart;         // The USART module

use core::default::Default;

#[no_mangle]
pub extern fn main() {
    // Acquire a USART with default configuration...
    let mut usart: Usart = Default::default();
    // ... and initialize its GPIO.
    usart.init_async();

    loop {
        // Perform a blocking read operation...
        let name = usart.read_line();
        // ... and echo back with a nice message.
        usart.write_line(&format!("Thank you {}!", name));
    }
}
