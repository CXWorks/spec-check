use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type UInt8 = u8;

pub type SbiErrorCode = i64;

pub spec const SBI_SUCCESS: SbiErrorCode = 0;

pub spec const SBI_ERR_FAILED: SbiErrorCode = (-1int) as i64;

pub spec const SBI_ERR_DENIED: SbiErrorCode = (-4int) as i64;

pub spec const byte: UInt8 = 0;

pub struct S {
    pub debug_console_enabled: bool,
}

pub open spec fn IsDebugConsoleWriteAllowed(s: S) -> bool;

pub open spec fn DebugConsoleIoError(s: S, b: UInt8) -> bool;

pub open spec fn ByteWrittenToDebugConsole(b: UInt8) -> bool;

pub open spec fn ResultEqual(result: SbiErrorCode, code: SbiErrorCode) -> bool;

} // verus!
