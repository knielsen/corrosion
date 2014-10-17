use chip::rom::getfun;

//*****************************************************************************
//
// Macro to generate an interrupt priority mask based on the number of bits
// of priority supported by the hardware.
//
//*****************************************************************************
pub const NUM_PRIORITY: u32 = 8;
pub const NUM_PRIORITY_BITS: uint = 3;
pub const PRIORITY_MASK: u32 = ((0xFF << (8 - NUM_PRIORITY_BITS)) & 0xFF);

pub fn Disable(interrupt: u32) { unsafe {
    let func = getfun(14, 3) as *const extern "C" fn(u32);
    (*func)(interrupt)
}}

pub fn Enable(interrupt: u32) { unsafe {
    let func = getfun(14, 0) as *const extern "C" fn(u32);
    (*func)(interrupt)
}}

pub fn IsEnabled(interrupt: u32) -> u32 { unsafe {
    let func = getfun(14, 12) as *const extern "C" fn(u32) -> u32;
    (*func)(interrupt)
}}

pub fn MasterDisable() -> bool { unsafe {
    let func = getfun(14, 2) as *const extern "C" fn() -> u8;
    (*func)() != 0
}}

pub fn MasterEnable() -> bool { unsafe {
    let func = getfun(14, 1) as *const extern "C" fn() -> u8;
    (*func)() != 0
}}

pub fn PendClear(interrupt: u32) { unsafe {
    let func = getfun(14, 9) as *const extern "C" fn(u32);
    (*func)(interrupt)
}}

pub fn PendSet(interrupt: u32) { unsafe {
    let func = getfun(14, 8) as *const extern "C" fn(u32);
    (*func)(interrupt)
}}

pub fn PriorityGet(interrupt: u32) -> i32 { unsafe {
    let func = getfun(14, 7) as *const extern "C" fn(u32) -> i32;
    (*func)(interrupt)
}}

pub fn PriorityGroupingGet() -> u32 { unsafe {
    let func = getfun(14, 5) as *const extern "C" fn() -> u32;
    (*func)()
}}

pub fn PriorityGroupingSet(bits: u32) { unsafe {
    let func = getfun(14, 4) as *const extern "C" fn(u32);
    (*func)(bits)
}}

pub fn PriorityMaskGet() -> u32 { unsafe {
    let func = getfun(14, 11) as *const extern "C" fn() -> u32;
    (*func)()
}}

pub fn PriorityMaskSet(mask: u32) { unsafe {
    let func = getfun(14, 10) as *const extern "C" fn(u32);
    (*func)(mask)
}}

pub fn PrioritySet(interrupt: u32, priority: u8) { unsafe {
    let func = getfun(14, 6) as *const extern "C" fn(u32, u8);
    (*func)(interrupt, priority)
}}

/*
extern void IntRegister(uint32_t ui32Interrupt, void (*pfnHandler)(void));
extern void IntUnregister(uint32_t ui32Interrupt);
extern void IntTrigger(uint32_t ui32Interrupt);
*/
