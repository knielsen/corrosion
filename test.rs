#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(globs)]
#![feature(asm)]
#![feature(macro_rules)]

extern crate core;
use core::prelude::*;
use chip::interrupts;
use chip::gpio;

#[path="src/tm4c123x/mod.rs"]
mod chip;

#[lang = "fail_fmt"]
fn fail_impl() -> ! {
    loop {}
}
#[lang = "stack_exhausted" ] fn stack_exhausted() {}
#[lang = "eh_personality" ] fn eh_personality() {}

const LED_RED: gpio::Pin = gpio::PIN1;
const LED_BLUE: gpio::Pin = gpio::PIN2;
const LED_GREEN: gpio::Pin = gpio::PIN3;

#[no_mangle]
pub extern "C" fn main() -> ! {
    interrupts::disable();
    gpio::enable(gpio::GPIO_PORTF_BASE);
    gpio::port_direction(gpio::GPIO_PORTF_BASE, 0x0e);

    let leds = [LED_RED, LED_GREEN, LED_BLUE];

    interrupts::enable();
    let mut x : uint = 0;
    loop {
    	let mut v : uint = 0;
        for i in leds.iter() {
	    if (x & (1 << v)) != 0 {
                i.high(gpio::GPIO_PORTF_BASE);
            };
	    v = v +1
        }
        for _ in range(0, 500000u) {
            chip::noop();
        }
        for i in leds.iter() {
            i.low(gpio::GPIO_PORTF_BASE);
        }
        for _ in range(0, 500000u) {
            chip::noop();
        }
	x = x + 1
    }
}
