use chip::rom::getfun;

//*****************************************************************************
//
// The following values define the bit field for the ui8Pins argument to
// several of the APIs.
//
//*****************************************************************************
pub const PIN_0: u8 = 0x01; // GPIO pin 0
pub const PIN_1: u8 = 0x02; // GPIO pin 1
pub const PIN_2: u8 = 0x04; // GPIO pin 2
pub const PIN_3: u8 = 0x08; // GPIO pin 3
pub const PIN_4: u8 = 0x10; // GPIO pin 4
pub const PIN_5: u8 = 0x20; // GPIO pin 5
pub const PIN_6: u8 = 0x40; // GPIO pin 6
pub const PIN_7: u8 = 0x80; // GPIO pin 7

//*****************************************************************************
//
// Values that can be passed to GPIODirModeSet as the ui32PinIO parameter, and
// returned from GPIODirModeGet.
//
//*****************************************************************************
pub const DIR_MODE_IN: u32 = 0x00000000;  // Pin is a GPIO input
pub const DIR_MODE_OUT: u32 = 0x00000001;  // Pin is a GPIO output
pub const DIR_MODE_HW: u32 = 0x00000002;  // Pin is a peripheral function

//*****************************************************************************
//
// Values that can be passed to GPIOIntTypeSet as the ui32IntType parameter,
// and returned from GPIOIntTypeGet.
//
//*****************************************************************************
pub const FALLING_EDGE: u32 = 0x00000000;  // Interrupt on falling edge
pub const RISING_EDGE: u32 = 0x00000004;  // Interrupt on rising edge
pub const BOTH_EDGES: u32 = 0x00000001;  // Interrupt on both edges
pub const LOW_LEVEL: u32 = 0x00000002;  // Interrupt on low level
pub const HIGH_LEVEL: u32 = 0x00000006;  // Interrupt on high level
pub const DISCRETE_INT: u32 = 0x00010000;  // Interrupt for individual pins

//*****************************************************************************
//
// Values that can be passed to GPIOPadConfigSet as the ui32Strength parameter,
// and returned by GPIOPadConfigGet in the *pui32Strength parameter.
//
//*****************************************************************************
pub const STRENGTH_2MA: u32 = 0x00000001;  // 2mA drive strength
pub const STRENGTH_4MA: u32 = 0x00000002;  // 4mA drive strength
pub const STRENGTH_6MA: u32 = 0x00000065;  // 6mA drive strength
pub const STRENGTH_8MA: u32 = 0x00000066;  // 8mA drive strength
pub const STRENGTH_8MA_SC: u32 = 0x0000006E;  // 8mA drive with slew rate control
pub const STRENGTH_10MA: u32 = 0x00000075;  // 10mA drive strength
pub const STRENGTH_12MA: u32 = 0x00000077;  // 12mA drive strength

//*****************************************************************************
//
// Values that can be passed to GPIOPadConfigSet as the ui32PadType parameter,
// and returned by GPIOPadConfigGet in the *pui32PadType parameter.
//
//*****************************************************************************
pub const PIN_TYPE_STD: u32 = 0x00000008;  // Push-pull
pub const PIN_TYPE_STD_WPU: u32 = 0x0000000A;  // Push-pull with weak pull-up
pub const PIN_TYPE_STD_WPD: u32 = 0x0000000C;  // Push-pull with weak pull-down
pub const PIN_TYPE_OD: u32 = 0x00000009;  // Open-drain
pub const PIN_TYPE_ANALOG: u32 = 0x00000000;  // Analog comparator
pub const PIN_TYPE_WAKE_HIGH: u32 = 0x00000208;  // Hibernate wake, high
pub const PIN_TYPE_WAKE_LOW: u32 = 0x00000108;  // Hibernate wake, low

//*****************************************************************************
//
// Values that can be passed to GPIOIntEnable() and GPIOIntDisable() functions
// in the ui32IntFlags parameter.
//
//*****************************************************************************
pub const INT_PIN_0: u32 = 0x00000001;
pub const INT_PIN_1: u32 = 0x00000002;
pub const INT_PIN_2: u32 = 0x00000004;
pub const INT_PIN_3: u32 = 0x00000008;
pub const INT_PIN_4: u32 = 0x00000010;
pub const INT_PIN_5: u32 = 0x00000020;
pub const INT_PIN_6: u32 = 0x00000040;
pub const INT_PIN_7: u32 = 0x00000080;
pub const INT_DMA: u32 = 0x00000100;

pub fn PinTypeGPIOOutput(port: u32, pins: u8) { unsafe {
    let func = getfun(4, 15) as *const extern "C" fn(u32, u8);
    (*func)(port, pins)
}}

pub fn PinWrite(port: u32, pins: u8, val: u8) { unsafe {
    let func = getfun(4, 0) as *const extern "C" fn(u32, u8, u8);
    (*func)(port, pins, val)
}}
