use vstd::prelude::*;

verus! {

pub type uint8_t = u8;
pub type unsigned_long = u64;
pub type UnsignedLong = u64;
pub type SbiErrorCode = i64;
pub type SBI_error_code = i64;

pub struct S {
    pub debug_console_write_allowed: bool,
    pub debug_console_io_error: bool,
    pub last_byte_written: u8,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_DENIED: i64 = -4;

pub open spec fn DebugConsoleWriteAllowed(s: S) -> bool;

pub open spec fn DebugConsoleIoError(s: S, byte: uint8_t) -> bool;

pub open spec fn DebugConsoleByteWritten(s: S, byte: uint8_t) -> bool;

pub open spec fn ResultEqual(result: i64, code: i64) -> bool;

} // verus!
