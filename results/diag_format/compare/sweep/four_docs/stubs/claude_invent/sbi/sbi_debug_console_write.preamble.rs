use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type Int64 = i64;

pub struct S {
    pub placeholder: int,
}

pub struct SbiRet {
    pub error: Int64,
    pub uvalue: UInt64,
}

pub const SBI_SUCCESS: Int64 = 0;
pub const SBI_ERR_FAILED: Int64 = -1;
pub const SBI_ERR_INVALID_PARAM: Int64 = -3;
pub const SBI_ERR_DENIED: Int64 = -4;

pub open spec fn SbiSharedMemoryValid(s: S, num_bytes: UInt64, base_addr_lo: UInt64, base_addr_hi: UInt64) -> bool;

pub open spec fn DebugConsoleWriteAllowed(s: S) -> bool;

pub open spec fn DebugConsoleIoError(s: S) -> bool;

pub open spec fn DebugConsoleBytesWritten(old_s: S, new_s: S, base_addr_lo: UInt64, base_addr_hi: UInt64, n: int) -> bool;

} // verus!
