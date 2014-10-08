#[inline]
pub fn noop() {
    unsafe{ asm!("nop"::::"volatile") }
}

pub mod interrupts {
    #[inline]
    pub fn enable() {
        unsafe{ asm!("cpsie i":::"memory") }
    }

    #[inline]
    pub fn disable() {
        unsafe{ asm!("cpsid i":::"memory") }
    }
}
