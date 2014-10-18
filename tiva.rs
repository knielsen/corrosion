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
use core::intrinsics::{volatile_load,volatile_store};
use chip::rom::GPIO;
use chip::rom::SysCtl;
use chip::rom::UART;
use chip::rom::USB;

#[cfg(chip_tm4c123gh6pm)]
#[allow(dead_code)]
#[macro_escape]
#[path="src/tm4c123gh6pm/mod.rs"]
mod chip;

#[lang = "fail_fmt"]         fn fail_impl() -> ! { loop {} }
#[lang = "stack_exhausted" ] fn stack_exhausted() {}
#[lang = "eh_personality" ]  fn eh_personality() {}

const RED:   u8 = GPIO::PIN_1;
const BLUE:  u8 = GPIO::PIN_2;
const GREEN: u8 = GPIO::PIN_3;

#[allow(non_snake_case)]
fn USBClockEnable(base: u32, div: u32, flags: u32) {
    unsafe {
        volatile_store((base + chip::usb::O_CC as u32) as *mut u32, (div-1) | flags);
    }
}

struct Uart {
    base: u32,
}

impl Uart {
    fn write(&self, data: &[u8]) {
        for ch in data.iter() {
            UART::CharPut(self.base, *ch);
        }
    }
}

//static S0: Uart = Uart{ base: chip::UART0_BASE };

#[no_mangle]
pub extern "C" fn main() -> ! {
    let s0: Uart = Uart{ base: chip::UART0_BASE };
    // Initialize clock
    SysCtl::ClockSet(
        SysCtl::SYSDIV_4 |
        SysCtl::USE_PLL |
        SysCtl::XTAL_16MHZ |
        SysCtl::OSC_MAIN);

    // Initialize LEDs
    SysCtl::PeripheralEnable(SysCtl::PERIPH_GPIOF);
    GPIO::PinTypeGPIOOutput(chip::GPIO_PORTF_BASE, RED|GREEN|BLUE);

    // Initialize serial port
    SysCtl::PeripheralEnable(SysCtl::PERIPH_UART0);
    SysCtl::PeripheralEnable(SysCtl::PERIPH_GPIOA);
    GPIO::PinConfigure(chip::pinmap::PA0_U0RX);
    GPIO::PinConfigure(chip::pinmap::PA1_U0TX);
    GPIO::PinTypeUART(chip::GPIO_PORTA_BASE, GPIO::PIN_0 | GPIO::PIN_1);
    UART::ConfigSetExpClk(chip::UART0_BASE, SysCtl::ClockGet(), 115200,
        UART::CONFIG_WLEN_8 |
        UART::CONFIG_STOP_ONE |
        UART::CONFIG_PAR_NONE);

    // Initialize USB
    SysCtl::PeripheralEnable(SysCtl::PERIPH_GPIOD);
    GPIO::PinTypeUSBAnalog(chip::GPIO_PORTD_BASE, GPIO::PIN_4 | GPIO::PIN_5);
    //SysCtl::PeripheralEnable(SysCtl::PERIPH_GPIOB);
    //GPIO::PinTypeUSBAnalog(chip::GPIO_PORTB_BASE, GPIO::PIN_0 | GPIO::PIN_1);

    // Reset the USB controller.
    SysCtl::PeripheralReset(SysCtl::PERIPH_USB0);
    // Enable Clocking to the USB controller.
    SysCtl::PeripheralEnable(SysCtl::PERIPH_USB0);
    // Turn on USB Phy clock.
    SysCtl::USBPLLEnable();
    // Set the PLL to USB clock divider.
    USBClockEnable(chip::USB0_BASE, 8, USB::CLOCK_INTERNAL);
    // Force device mode
    USB::DevMode(chip::USB0_BASE);
    // Get the current interrupt status to clear all pending USB
    // interrupts.
    USB::IntStatusControl(chip::USB0_BASE);
    USB::IntStatusEndpoint(chip::USB0_BASE);
    // Enable USB Interrupts.
    USB::IntEnableControl(chip::USB0_BASE,
                          USB::INTCTRL_RESET |
                          USB::INTCTRL_DISCONNECT |
                          USB::INTCTRL_RESUME |
                          USB::INTCTRL_SUSPEND |
                          USB::INTCTRL_SOF);
    USB::IntEnableEndpoint(chip::USB0_BASE, USB::INTEP_ALL);
    // Attach the device using the soft connect.
    USB::DevConnect(chip::USB0_BASE);

    chip::interrupts::enable();
    let mut line = [ 0, 0, /* '\r' as u8, */ '\n' as u8 ];
    loop {
        GPIO::PinWrite(chip::GPIO_PORTF_BASE, RED|GREEN|BLUE, GREEN);
        SysCtl::Delay(1000000);
        GPIO::PinWrite(chip::GPIO_PORTF_BASE, RED|GREEN|BLUE, 0);
        SysCtl::Delay(10000000);

        let mut txlevel = 0;
        let mut rxlevel = 0;
        UART::FIFOLevelGet(chip::UART0_BASE, &mut txlevel, &mut rxlevel);

        line[0] = match txlevel {
            UART::FIFO_TX1_8 => '1',
            UART::FIFO_TX2_8 => '2',
            UART::FIFO_TX4_8 => '4',
            UART::FIFO_TX6_8 => '6',
            UART::FIFO_TX7_8 => '7',
            _ => '!',
        } as u8;
        line[1] = match rxlevel {
            UART::FIFO_RX1_8 => '1',
            UART::FIFO_RX2_8 => '2',
            UART::FIFO_RX4_8 => '4',
            UART::FIFO_RX6_8 => '6',
            UART::FIFO_RX7_8 => '7',
            _ => '!',
        } as u8;

        s0.write(&line);
    }
}
