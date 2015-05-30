#![no_std]
#![no_main]
#![feature(no_std)]
extern crate emlib;         // Include emlib bindings
extern crate modules;       // Include `REM'

use emlib::gpio::Port;
use modules::{GpioPin, Button, Led};

const PB0: &'static Button = &GpioPin { port: Port::B, pin: 9 };
const PB1: &'static Button = &GpioPin { port: Port::B, pin: 10 };
const LED0: &'static Led = &GpioPin { port: Port::E, pin: 2 };
const LED1: &'static Led = &GpioPin { port: Port::E, pin: 3 };

fn blink_led(pin: u8) {
    match pin {
        9  => LED0.toggle(),
        10 => LED1.toggle(),
        _  => ()
    }
}

#[no_mangle]
pub extern fn main() {
    PB0.init();
    PB1.init();
    LED0.init();
    LED1.init();

    PB0.on_click(blink_led);
    PB1.on_click(blink_led);
    loop {}
}
