use vstd::prelude::*;
verus! {

pub struct SbiRet {
    pub error: i64,
    pub uvalue: u64,
}

pub struct S {
    pub console_output: Seq<u8>,
    pub debug_console_enabled: bool,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_DENIED: i64 = -4;

pub open spec fn DebugConsoleWriteAllowed(s: S) -> bool;

pub open spec fn DebugConsoleIoError(s: S, byte: u8) -> bool;

pub open spec fn DebugConsoleByteWritten(old_s: S, new_s: S, byte: u8) -> bool;

} // verus!
