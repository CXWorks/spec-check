use vstd::prelude::*;

verus! {

pub type UInt = u64;

pub type SbiErrorCode = i64;

pub type PhysAddress = u128;

pub struct S {
    pub dummy: u64,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_DENIED: SbiErrorCode = -4;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub open spec fn num_bytes(s: S) -> UInt;

pub open spec fn base_addr_lo(s: S) -> UInt;

pub open spec fn base_addr_hi(s: S) -> UInt;

pub open spec fn IsValidSharedMemory(s: S, num_bytes: UInt, lo: UInt, hi: UInt) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn DebugConsoleWriteAllowed(s: S) -> bool;

pub open spec fn DebugConsoleIoError(s: S) -> bool;

pub open spec fn PhysAddr(hi: UInt, lo: UInt) -> PhysAddress;

pub open spec fn DebugConsoleWritten(addr: PhysAddress, n: UInt) -> bool;

} // verus!
