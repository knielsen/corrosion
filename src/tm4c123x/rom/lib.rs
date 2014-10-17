#[inline]
unsafe fn getfun(table_offset: uint, fun_offset: uint) -> uint {
    let table = *((0x1000010u + 4*table_offset) as *const uint);
    table + 4*fun_offset
}

pub mod SysCtl;
pub mod GPIO;
pub mod UART;
pub mod USB;
