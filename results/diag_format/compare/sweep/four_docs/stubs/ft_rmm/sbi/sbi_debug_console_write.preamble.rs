use vstd::prelude::*;
verus! {

pub type UInt = u64;
pub type Bits = u64;
pub type SbiErrorCode = i64;

pub struct S {
    pub dummy: u64,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_DENIED: SbiErrorCode = -4;

pub open spec fn InputMemoryMeetsSection3_2(s: S, num_bytes: UInt, base_addr_lo: Bits, base_addr_hi: Bits) -> bool;

pub open spec fn DebugConsoleWriteAllowed(s: S) -> bool;

pub open spec fn DebugConsoleIoError(s: S) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn NumBytesWrittenToDebugConsole(s: S) -> UInt;

} // verus!
