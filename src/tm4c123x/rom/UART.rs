use chip::rom::getfun;

//*****************************************************************************
//
// Values that can be passed to UARTIntEnable, UARTIntDisable, and UARTIntClear
// as the ui32IntFlags parameter, and returned from UARTIntStatus.
//
//*****************************************************************************
pub const INT_DMATX: u32 = 0x20000;     // DMA TX interrupt
pub const INT_DMARX: u32 = 0x10000;     // DMA RX interrupt
pub const INT_9BIT:  u32 = 0x1000;      // 9-bit address match interrupt
pub const INT_OE:    u32 = 0x400;       // Overrun Error Interrupt Mask
pub const INT_BE:    u32 = 0x200;       // Break Error Interrupt Mask
pub const INT_PE:    u32 = 0x100;       // Parity Error Interrupt Mask
pub const INT_FE:    u32 = 0x080;       // Framing Error Interrupt Mask
pub const INT_RT:    u32 = 0x040;       // Receive Timeout Interrupt Mask
pub const INT_TX:    u32 = 0x020;       // Transmit Interrupt Mask
pub const INT_RX:    u32 = 0x010;       // Receive Interrupt Mask
pub const INT_DSR:   u32 = 0x008;       // DSR Modem Interrupt Mask
pub const INT_DCD:   u32 = 0x004;       // DCD Modem Interrupt Mask
pub const INT_CTS:   u32 = 0x002;       // CTS Modem Interrupt Mask
pub const INT_RI:    u32 = 0x001;       // RI Modem Interrupt Mask

//*****************************************************************************
//
// Values that can be passed to UARTConfigSetExpClk as the ui32Config parameter
// and returned by UARTConfigGetExpClk in the pui32Config parameter.
// Additionally, the UART_CONFIG_PAR_* subset can be passed to
// UARTParityModeSet as the ui32Parity parameter, and are returned by
// UARTParityModeGet.
//
//*****************************************************************************
pub const CONFIG_WLEN_MASK: u32 = 0x00000060;  // Mask for extracting word length
pub const CONFIG_WLEN_8:    u32 = 0x00000060;  // 8 bit data
pub const CONFIG_WLEN_7:    u32 = 0x00000040;  // 7 bit data
pub const CONFIG_WLEN_6:    u32 = 0x00000020;  // 6 bit data
pub const CONFIG_WLEN_5:    u32 = 0x00000000;  // 5 bit data
pub const CONFIG_STOP_MASK: u32 = 0x00000008;  // Mask for extracting stop bits
pub const CONFIG_STOP_ONE:  u32 = 0x00000000;  // One stop bit
pub const CONFIG_STOP_TWO:  u32 = 0x00000008;  // Two stop bits
pub const CONFIG_PAR_MASK:  u32 = 0x00000086;  // Mask for extracting parity
pub const CONFIG_PAR_NONE:  u32 = 0x00000000;  // No parity
pub const CONFIG_PAR_EVEN:  u32 = 0x00000006;  // Even parity
pub const CONFIG_PAR_ODD:   u32 = 0x00000002;  // Odd parity
pub const CONFIG_PAR_ONE:   u32 = 0x00000082;  // Parity bit is one
pub const CONFIG_PAR_ZERO:  u32 = 0x00000086;  // Parity bit is zero

//*****************************************************************************
//
// Values that can be passed to UARTFIFOLevelSet as the ui32TxLevel parameter
// and returned by UARTFIFOLevelGet in the pui32TxLevel.
//
//*****************************************************************************
pub const FIFO_TX1_8: u32 = 0x00000000;  // Transmit interrupt at 1/8 Full
pub const FIFO_TX2_8: u32 = 0x00000001;  // Transmit interrupt at 1/4 Full
pub const FIFO_TX4_8: u32 = 0x00000002;  // Transmit interrupt at 1/2 Full
pub const FIFO_TX6_8: u32 = 0x00000003;  // Transmit interrupt at 3/4 Full
pub const FIFO_TX7_8: u32 = 0x00000004;  // Transmit interrupt at 7/8 Full

//*****************************************************************************
//
// Values that can be passed to UARTFIFOLevelSet as the ui32RxLevel parameter
// and returned by UARTFIFOLevelGet in the pui32RxLevel.
//
//*****************************************************************************
pub const FIFO_RX1_8: u32 = 0x00000000;  // Receive interrupt at 1/8 Full
pub const FIFO_RX2_8: u32 = 0x00000008;  // Receive interrupt at 1/4 Full
pub const FIFO_RX4_8: u32 = 0x00000010;  // Receive interrupt at 1/2 Full
pub const FIFO_RX6_8: u32 = 0x00000018;  // Receive interrupt at 3/4 Full
pub const FIFO_RX7_8: u32 = 0x00000020;  // Receive interrupt at 7/8 Full

//*****************************************************************************
//
// Values that can be passed to UARTDMAEnable() and UARTDMADisable().
//
//*****************************************************************************
pub const DMA_ERR_RXSTOP: u32 = 0x00000004;  // Stop DMA receive if UART error
pub const DMA_TX:         u32 = 0x00000002;  // Enable DMA for transmit
pub const DMA_RX:         u32 = 0x00000001;  // Enable DMA for receive

//*****************************************************************************
//
// Values returned from UARTRxErrorGet().
//
//*****************************************************************************
pub const RXERROR_OVERRUN: u32 = 0x00000008;
pub const RXERROR_BREAK:   u32 = 0x00000004;
pub const RXERROR_PARITY:  u32 = 0x00000002;
pub const RXERROR_FRAMING: u32 = 0x00000001;

//*****************************************************************************
//
// Values that can be passed to UARTHandshakeOutputsSet() or returned from
// UARTHandshakeOutputGet().
//
//*****************************************************************************
pub const OUTPUT_RTS: u32 = 0x00000800;
pub const OUTPUT_DTR: u32 = 0x00000400;

//*****************************************************************************
//
// Values that can be returned from UARTHandshakeInputsGet().
//
//*****************************************************************************
pub const INPUT_RI:  u32 = 0x00000100;
pub const INPUT_DCD: u32 = 0x00000004;
pub const INPUT_DSR: u32 = 0x00000002;
pub const INPUT_CTS: u32 = 0x00000001;

//*****************************************************************************
//
// Values that can be passed to UARTFlowControl() or returned from
// UARTFlowControlGet().
//
//*****************************************************************************
pub const FLOWCONTROL_TX:   u32 = 0x00008000;
pub const FLOWCONTROL_RX:   u32 = 0x00004000;
pub const FLOWCONTROL_NONE: u32 = 0x00000000;

//*****************************************************************************
//
// Values that can be passed to UARTTxIntModeSet() or returned from
// UARTTxIntModeGet().
//
//*****************************************************************************
pub const TXINT_MODE_FIFO: u32 = 0x00000000;
pub const TXINT_MODE_EOT:  u32 = 0x00000010;

//*****************************************************************************
//
// Values that can be passed to UARTClockSourceSet() or returned from
// UARTClockSourceGet().
//
//*****************************************************************************
pub const CLOCK_SYSTEM: u32 = 0x00000000;
pub const CLOCK_PIOSC:  u32 = 0x00000005;

pub fn _9BitAddrSend(base: u32, addr: u8) { unsafe {
    let func = getfun(1, 36) as *const extern "C" fn(u32, u8);
    (*func)(base, addr)
}}

pub fn _9BitAddrSet(base: u32, addr: u8, mask: u8) { unsafe {
    let func = getfun(1, 35) as *const extern "C" fn(u32, u8, u8);
    (*func)(base, addr, mask)
}}

pub fn _9BitDisable(base: u32) { unsafe {
    let func = getfun(1, 34) as *const extern "C" fn(u32);
    (*func)(base)
}}

pub fn _9BitEnable(base: u32) { unsafe {
    let func = getfun(1, 33) as *const extern "C" fn(u32);
    (*func)(base)
}}

pub fn BreakCtl(base: u32, state: bool) { unsafe {
    let func = getfun(1, 16) as *const extern "C" fn(u32, u8);
    (*func)(base, state as u8)
}}

pub fn Busy(base: u32) -> bool { unsafe {
    let func = getfun(1, 26) as *const extern "C" fn(u32) -> u8;
    (*func)(base) != 0
}}

pub fn CharGet(base: u32) -> i32 { unsafe {
    let func = getfun(1, 14) as *const extern "C" fn(u32) -> i32;
    (*func)(base)
}}

pub fn CharGetNonBlocking(base: u32) -> i32 { unsafe {
    let func = getfun(1, 13) as *const extern "C" fn(u32) -> i32;
    (*func)(base)
}}

pub fn CharPut(base: u32, data: u8) { unsafe {
    let func = getfun(1, 0) as *const extern "C" fn(u32, u8);
    (*func)(base, data)
}}

pub fn CharPutNonBlocking(base: u32, data: u8) { unsafe {
    let func = getfun(1, 15) as *const extern "C" fn(u32, u8);
    (*func)(base, data)
}}

pub fn CharsAvail(base: u32) -> bool { unsafe {
    let func = getfun(1, 11) as *const extern "C" fn(u32) -> u8;
    (*func)(base) != 0
}}

pub fn ClockSourceGet(base: u32) -> u32 { unsafe {
    let func = getfun(1, 32) as *const extern "C" fn(u32) -> u32;
    (*func)(base)
}}

pub fn ClockSourceSet(base: u32, source: u32) { unsafe {
    let func = getfun(1, 31) as *const extern "C" fn(u32, u32);
    (*func)(base, source)
}}

pub fn ConfigGetExpClk(base: u32, clk: u32, baud: *mut u32, config: *mut u32) { unsafe {
    let func = getfun(1, 6) as *const extern "C" fn(u32, u32, *mut u32, *mut u32);
    (*func)(base, clk, baud, config)
}}

pub fn ConfigSetExpClk(base: u32, clk: u32, baud: u32, config: u32) { unsafe {
    let func = getfun(1, 5) as *const extern "C" fn(u32, u32, u32, u32);
    (*func)(base, clk, baud, config)
}}

pub fn Disable(base: u32) { unsafe {
    let func = getfun(1, 8) as *const extern "C" fn(u32);
    (*func)(base)
}}

pub fn DisableSIR(base: u32) { unsafe {
    let func = getfun(1, 10) as *const extern "C" fn(u32);
    (*func)(base)
}}

pub fn DMADisable(base: u32, flags: u32) { unsafe {
    let func = getfun(1, 23) as *const extern "C" fn(u32, u32);
    (*func)(base, flags)
}}

pub fn DMAEnable(base: u32, flags: u32) { unsafe {
    let func = getfun(1, 22) as *const extern "C" fn(u32, u32);
    (*func)(base, flags)
}}

pub fn Enable(base: u32) { unsafe {
    let func = getfun(1, 7) as *const extern "C" fn(u32);
    (*func)(base)
}}

pub fn EnableSIR(base: u32, lowpower: bool) { unsafe {
    let func = getfun(1, 9) as *const extern "C" fn(u32, u8);
    (*func)(base, lowpower as u8)
}}

pub fn FIFODisable(base: u32) { unsafe {
    let func = getfun(1, 25) as *const extern "C" fn(u32);
    (*func)(base)
}}

pub fn FIFOEnable(base: u32) { unsafe {
    let func = getfun(1, 24) as *const extern "C" fn(u32);
    (*func)(base)
}}

pub fn FIFOLevelGet(base: u32, txlevel: &mut u32, rxlevel: &mut u32) { unsafe {
    let func = getfun(1, 4) as *const extern "C" fn(u32, *mut u32, *mut u32);
    (*func)(base, txlevel, rxlevel)
}}

pub fn FIFOLevelSet(base: u32, txlevel: u32, rxlevel: u32) { unsafe {
    let func = getfun(1, 3) as *const extern "C" fn(u32, u32, u32);
    (*func)(base, txlevel, rxlevel)
}}

pub fn IntClear(base: u32, flags: u32) { unsafe {
    let func = getfun(1, 20) as *const extern "C" fn(u32, u32);
    (*func)(base, flags)
}}

pub fn IntDisable(base: u32, flags: u32) { unsafe {
    let func = getfun(1, 18) as *const extern "C" fn(u32, u32);
    (*func)(base, flags)
}}

pub fn IntEnable(base: u32, flags: u32) { unsafe {
    let func = getfun(1, 17) as *const extern "C" fn(u32, u32);
    (*func)(base, flags)
}}

pub fn IntStatus(base: u32, masked: bool) -> u32 { unsafe {
    let func = getfun(1, 19) as *const extern "C" fn(u32, u8) -> u32;
    (*func)(base, masked as u8)
}}

pub fn ParityModeGet(base: u32) -> u32 { unsafe {
    let func = getfun(1, 2) as *const extern "C" fn(u32) -> u32;
    (*func)(base)
}}

pub fn ParityModeSet(base: u32, parity: u32) { unsafe {
    let func = getfun(1, 1) as *const extern "C" fn(u32, u32);
    (*func)(base, parity)
}}

pub fn RxErrorClear(base: u32) { unsafe {
    let func = getfun(1, 30) as *const extern "C" fn(u32);
    (*func)(base)
}}

pub fn RxErrorGet(base: u32) -> u32 { unsafe {
    let func = getfun(1, 29) as *const extern "C" fn(u32) -> u32;
    (*func)(base)
}}

pub fn SpaceAvail(base: u32) -> bool { unsafe {
    let func = getfun(1, 12) as *const extern "C" fn(u32) -> u8;
    (*func)(base) != 0
}}

pub fn TxIntModeGet(base: u32) -> u32 { unsafe {
    let func = getfun(1, 28) as *const extern "C" fn(u32) -> u32;
    (*func)(base)
}}

pub fn TxIntModeSet(base: u32, mode: u32) { unsafe {
    let func = getfun(1, 27) as *const extern "C" fn(u32, u32);
    (*func)(base, mode)
}}

pub fn Update() { unsafe {
    let func = getfun(1, 21) as *const extern "C" fn();
    (*func)()
}}
