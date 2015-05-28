#![no_std]
#![feature(no_std, core, collections)]

#[macro_use]
extern crate core;
extern crate libc;
extern crate collections;

extern crate emlib;
extern crate emdrv;

pub use self::gpio::{Button, Led, GpioPin};
pub use self::usart::{Usart, Config, Location};

pub mod adc;

pub mod dma;

mod gpio;

mod usart;
