use vstd::prelude::*;
verus! {

pub type UInt8 = u8;
pub type UInt64 = u64;
pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_DENIED: SbiErrorCode = -4;

pub struct S {
    pub debug_console_write_allowed: bool,
    pub debug_console_io_error: bool,
    pub last_written_byte: u8,
}

pub open spec fn IsDebugConsoleWriteAllowed(s: S) -> bool;

pub open spec fn DebugConsoleIoError(s: S, byte: UInt8) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn ByteWrittenToDebugConsole(s: S, byte: UInt8) -> bool;

} // verus!
