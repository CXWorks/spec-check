use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Address = u64;
pub type SbiErrorCode = i64;

pub struct S {
    pub dummy: int,
}

pub spec const SBI_SUCCESS: SbiErrorCode = 0;
pub spec const SBI_ERR_FAILED: SbiErrorCode = (-1) as i64;
pub spec const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = (-2) as i64;
pub spec const SBI_ERR_INVALID_PARAM: SbiErrorCode = (-3) as i64;
pub spec const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = (-5) as i64;
pub spec const SBI_ERR_BAD_RANGE: SbiErrorCode = (-11) as i64;

pub spec const XLEN: UInt32 = 64;

#[allow(non_upper_case_globals)]
pub spec const i: UInt32 = 0;

pub open spec fn IsValidEventId(s: S, event_id: UInt32) -> bool;
pub open spec fn IsReservedEventId(s: S, event_id: UInt32) -> bool;
pub open spec fn PlatformSupportsEvent(s: S, event_id: UInt32) -> bool;
pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;
pub open spec fn Exists(v: UInt32, range: bool, pred: bool) -> bool;
pub open spec fn ForAll(v: UInt32, range: bool, pred: bool) -> bool;
pub open spec fn IsReservedAttrId(s: S, attr_id: int) -> bool;
pub open spec fn SatisfiesSharedMemoryRequirements(s: S, lo: Address, hi: Address, size: int) -> bool;
pub open spec fn ReadFailedForOtherReason(s: S) -> bool;
pub open spec fn SharedMemoryAt(s: S, addr: int) -> int;
pub open spec fn OutputPhysAddr(lo: Address, hi: Address) -> int;
pub open spec fn EventAttr(s: S, event_id: UInt32, attr_id: int) -> int;

} // verus!
