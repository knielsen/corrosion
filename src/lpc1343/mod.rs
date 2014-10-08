pub use chip::arm::noop;
pub use chip::arm::interrupts;

#[path="../arm.rs"]
#[allow(dead_code)]
mod arm;

#[allow(dead_code)]
pub mod gpio {
    use core::intrinsics::{volatile_load,volatile_store};

    pub const GPIOBASE: uint = 0x50000000;
    pub const GPIOPLEN: uint = 0x10000;
    const GPIODATA: uint = 0x3ffc;
    const GPIODIR:  uint = 0x8000;
    const GPIOIS:   uint = 0x8004;
    const GPIOIBE:  uint = 0x8008;
    const GPIOIEV:  uint = 0x800c;
    const GPIOIE:   uint = 0x8010;
    const GPIORIS:  uint = 0x8014;
    const GPIOMIS:  uint = 0x8018;
    const GPIOIC:   uint = 0x801c;

    #[inline]
    fn set(port: uint, offset: uint, value: u32) {
        unsafe {
            volatile_store((GPIOBASE + port*GPIOPLEN + offset) as *mut u32,
                value)
        }
    }

    #[inline]
    fn get(port: uint, offset: uint) -> u32 {
        unsafe {
            volatile_load((GPIOBASE + port*GPIOPLEN + offset) as *const u32)
        }
    }

    #[inline]
    pub fn port_direction(port: uint, mask: u32) {
        set(port, GPIODIR, mask)
    }

    pub struct Pin {
        nr: u32,
    }

    impl Pin {
        fn basemask(self) -> (uint, u32) {
            let n = self.nr as uint;
            (GPIOBASE + (n / 12) * GPIOPLEN, 1u32 << (2 + (n % 12)))
        }

        pub fn input(self) {
            let (base, mask) = self.basemask();
            unsafe {
                let v = volatile_load((base + GPIODIR) as *const u32);
                volatile_store((base + GPIODIR) as *mut u32, v & !mask);
            }
        }

        pub fn output(self) {
            let (base, mask) = self.basemask();
            unsafe {
                let v = volatile_load((base + GPIODIR) as *const u32);
                volatile_store((base + GPIODIR) as *mut u32, v | mask);
            }
        }

        pub fn high(self) {
            let (base, mask) = self.basemask();
            unsafe {
                volatile_store((base + mask as uint) as *mut u32, 0xFFF)
            }
        }

        pub fn low(&self) {
            let (base, mask) = self.basemask();
            unsafe {
                volatile_store((base + mask as uint) as *mut u32, 0x0)
            }
        }
    }

    pub const PIN0_0:  Pin = Pin{ nr: 0*12 + 0 };
    pub const PIN0_1:  Pin = Pin{ nr: 0*12 + 1 };
    pub const PIN0_2:  Pin = Pin{ nr: 0*12 + 2 };
    pub const PIN0_3:  Pin = Pin{ nr: 0*12 + 3 };
    pub const PIN0_4:  Pin = Pin{ nr: 0*12 + 4 };
    pub const PIN0_5:  Pin = Pin{ nr: 0*12 + 5 };
    pub const PIN0_6:  Pin = Pin{ nr: 0*12 + 6 };
    pub const PIN0_7:  Pin = Pin{ nr: 0*12 + 7 };
    pub const PIN0_8:  Pin = Pin{ nr: 0*12 + 8 };
    pub const PIN0_9:  Pin = Pin{ nr: 0*12 + 9 };
    pub const PIN0_10: Pin = Pin{ nr: 0*12 + 10 };
    pub const PIN0_11: Pin = Pin{ nr: 0*12 + 11 };

    pub const PIN1_0:  Pin = Pin{ nr: 1*12 + 0 };
    pub const PIN1_1:  Pin = Pin{ nr: 1*12 + 1 };
    pub const PIN1_2:  Pin = Pin{ nr: 1*12 + 2 };
    pub const PIN1_3:  Pin = Pin{ nr: 1*12 + 3 };
    pub const PIN1_4:  Pin = Pin{ nr: 1*12 + 4 };
    pub const PIN1_5:  Pin = Pin{ nr: 1*12 + 5 };
    pub const PIN1_6:  Pin = Pin{ nr: 1*12 + 6 };
    pub const PIN1_7:  Pin = Pin{ nr: 1*12 + 7 };
    pub const PIN1_8:  Pin = Pin{ nr: 1*12 + 8 };
    pub const PIN1_9:  Pin = Pin{ nr: 1*12 + 9 };
    pub const PIN1_10: Pin = Pin{ nr: 1*12 + 10 };
    pub const PIN1_11: Pin = Pin{ nr: 1*12 + 11 };

    pub const PIN2_0:  Pin = Pin{ nr: 2*12 + 0 };
    pub const PIN2_1:  Pin = Pin{ nr: 2*12 + 1 };
    pub const PIN2_2:  Pin = Pin{ nr: 2*12 + 2 };
    pub const PIN2_3:  Pin = Pin{ nr: 2*12 + 3 };
    pub const PIN2_4:  Pin = Pin{ nr: 2*12 + 4 };
    pub const PIN2_5:  Pin = Pin{ nr: 2*12 + 5 };
    pub const PIN2_6:  Pin = Pin{ nr: 2*12 + 6 };
    pub const PIN2_7:  Pin = Pin{ nr: 2*12 + 7 };
    pub const PIN2_8:  Pin = Pin{ nr: 2*12 + 8 };
    pub const PIN2_9:  Pin = Pin{ nr: 2*12 + 9 };
    pub const PIN2_10: Pin = Pin{ nr: 2*12 + 10 };
    pub const PIN2_11: Pin = Pin{ nr: 2*12 + 11 };

    pub const PIN3_0:  Pin = Pin{ nr: 3*12 + 0 };
    pub const PIN3_1:  Pin = Pin{ nr: 3*12 + 1 };
    pub const PIN3_2:  Pin = Pin{ nr: 3*12 + 2 };
    pub const PIN3_3:  Pin = Pin{ nr: 3*12 + 3 };
    pub const PIN3_4:  Pin = Pin{ nr: 3*12 + 4 };
    pub const PIN3_5:  Pin = Pin{ nr: 3*12 + 5 };
    pub const PIN3_6:  Pin = Pin{ nr: 3*12 + 6 };
    pub const PIN3_7:  Pin = Pin{ nr: 3*12 + 7 };
    pub const PIN3_8:  Pin = Pin{ nr: 3*12 + 8 };
    pub const PIN3_9:  Pin = Pin{ nr: 3*12 + 9 };
    pub const PIN3_10: Pin = Pin{ nr: 3*12 + 10 };
    pub const PIN3_11: Pin = Pin{ nr: 3*12 + 11 };
}
