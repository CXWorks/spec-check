use vstd::prelude::*;
verus! {

pub type UInt = u64;
pub type Address = u64;
pub type PhysicalAddress = int;
pub type SbiErrorCode = int;

pub struct S {
    pub dummy: int,
}

pub spec const SBI_SUCCESS: SbiErrorCode = 0;
pub spec const SBI_ERR_FAILED: SbiErrorCode = -1;
pub spec const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub spec const SBI_ERR_DENIED: SbiErrorCode = -4;

pub uninterp spec fn IsValidSharedMemory(s: S, num_bytes: UInt, base_addr_lo: Address, base_addr_hi: Address) -> bool;

pub uninterp spec fn ResultEqual(result: SbiErrorCode, code: SbiErrorCode) -> bool;

pub uninterp spec fn DebugConsoleWriteAllowed(s: S) -> bool;

pub uninterp spec fn DebugConsoleIoError(s: S) -> bool;

pub uninterp spec fn PhysAddr(hi: Address, lo: Address) -> PhysicalAddress;

pub uninterp spec fn DebugConsoleWritten(s: S, addr: PhysicalAddress, num_bytes: UInt) -> bool;

} // verus!
