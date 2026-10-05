use vstd::prelude::*;
verus! {

pub type UInt = u64;
pub type Bits = u64;
pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_DENIED: SbiErrorCode = -4;

pub struct DebugConsoleState {
    pub output: Seq<u8>,
}

pub struct S {
    pub DebugConsole: DebugConsoleState,
}

pub open spec fn InputMemoryMeetsSection3_2(num_bytes: UInt, base_addr_lo: Bits, base_addr_hi: Bits) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn DebugConsoleWriteAllowed() -> bool;

pub open spec fn DebugConsoleIoError() -> bool;

pub open spec fn NumBytesWrittenToDebugConsole() -> UInt;

} // verus!
