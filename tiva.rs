#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(globs)]
#![feature(asm)]
#![feature(macro_rules)]
#![feature(phase)]

#[phase(link,plugin)]
extern crate core;
use core::prelude::*;
use chip::rom::SysCtl;
use chip::rom::GPIO;
use chip::rom::UART;

#[cfg(chip_tm4c123gh6pm)]
#[allow(dead_code)]
#[path="src/tm4c123gh6pm/mod.rs"]
mod chip;

#[lang = "fail_fmt"]
fn fail_impl() -> ! {
    loop {}
}
#[lang = "stack_exhausted" ] fn stack_exhausted() {}
#[lang = "eh_personality" ] fn eh_personality() {}

const RED:   u8 = GPIO::PIN_1;
const GREEN: u8 = GPIO::PIN_2;
const BLUE:  u8 = GPIO::PIN_3;

#[no_mangle]
pub extern "C" fn main() -> ! {
    SysCtl::ClockSet(
        SysCtl::SYSDIV_4 |
        SysCtl::USE_PLL |
        SysCtl::XTAL_16MHZ |
        SysCtl::OSC_MAIN);

    SysCtl::PeripheralEnable(SysCtl::PERIPH_GPIOF);
    GPIO::PinTypeGPIOOutput(chip::GPIO_PORTF_BASE, RED|GREEN|BLUE);

    SysCtl::PeripheralEnable(SysCtl::PERIPH_UART0);
    SysCtl::PeripheralEnable(SysCtl::PERIPH_GPIOA);
    GPIO::PinConfigure(chip::pinmap::PA0_U0RX);
    GPIO::PinConfigure(chip::pinmap::PA1_U0TX);
    GPIO::PinTypeUART(chip::GPIO_PORTA_BASE, GPIO::PIN_0 | GPIO::PIN_1);
    UART::ConfigSetExpClk(chip::UART0_BASE, SysCtl::ClockGet(), 115200,
        UART::CONFIG_WLEN_8 |
        UART::CONFIG_STOP_ONE |
        UART::CONFIG_PAR_NONE);

    loop {
        GPIO::PinWrite(chip::GPIO_PORTF_BASE, RED|GREEN|BLUE, RED|GREEN|BLUE);
        SysCtl::Delay(500000);
        GPIO::PinWrite(chip::GPIO_PORTF_BASE, RED|GREEN|BLUE, 0);
        SysCtl::Delay(5000000);

        UART::CharPut(chip::UART0_BASE, '!' as u8);
    }
}
