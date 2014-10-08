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

#[path="src/lpc1343/mod.rs"]
mod chip;

#[lang = "fail_fmt"]
fn fail_impl() -> ! {
    loop {}
}
#[lang = "stack_exhausted" ] fn stack_exhausted() {}
#[lang = "eh_personality" ] fn eh_personality() {}

const LED0: gpio::Pin = gpio::PIN3_0;
const LED1: gpio::Pin = gpio::PIN3_1;
const LED2: gpio::Pin = gpio::PIN3_2;
const LED3: gpio::Pin = gpio::PIN3_3;
const LED4: gpio::Pin = gpio::PIN2_4;
const LED5: gpio::Pin = gpio::PIN2_5;
const LED6: gpio::Pin = gpio::PIN2_6;
const LED7: gpio::Pin = gpio::PIN2_7;

#[no_mangle]
pub extern "C" fn main() -> ! {
    interrupts::disable();
    gpio::port_direction(3, 0x0F);
    gpio::port_direction(2, 0xF0);

    let leds = [LED0, LED1, LED2, LED3,
                LED4, LED5, LED6, LED7];

    interrupts::enable();
    loop {
        for i in leds.iter() {
            i.high();
        }
        for _ in range(0, 100000u) {
            chip::noop();
        }
        for i in leds.iter() {
            i.low();
        }
        for _ in range(0, 100000u) {
            chip::noop();
        }
    }
}
