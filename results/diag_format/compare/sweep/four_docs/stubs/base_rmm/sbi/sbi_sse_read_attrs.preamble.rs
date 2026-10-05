use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt = u64;
pub type SbiErrorCode = i64;

pub struct S {
    pub XLEN: u64,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = -5;
pub const SBI_ERR_BAD_RANGE: SbiErrorCode = -11;

#[allow(non_upper_case_globals)]
pub const i: u32 = 0;

pub open spec fn Exists(var: u32, range: bool, body: bool) -> bool;

pub open spec fn ForAll(var: u32, range: bool, body: bool) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn IsValidEventId(event_id: UInt32) -> bool;

pub open spec fn IsReservedEventId(event_id: UInt32) -> bool;

pub open spec fn PlatformSupportsEvent(event_id: UInt32) -> bool;

pub open spec fn IsReservedAttrId(attr_id: int) -> bool;

pub open spec fn SatisfiesSharedMemoryRequirements(lo: UInt, hi: UInt, size: int) -> bool;

pub open spec fn ReadFailedForOtherReason() -> bool;

pub open spec fn SharedMemoryAt(addr: int) -> int;

pub open spec fn OutputPhysAddr(lo: UInt, hi: UInt) -> int;

pub open spec fn EventAttr(event_id: UInt32, attr_id: int) -> int;

} // verus!
