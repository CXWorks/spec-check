use vstd::prelude::*;

verus! {

pub type UInt32 = u64;
pub type UInt64 = u64;
pub type SbiErrorCode = i64;

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = -5;
pub const SBI_ERR_BAD_RANGE: SbiErrorCode = -11;

pub const XLEN: u64 = 64;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn IsReservedEventId(event_id: UInt32) -> bool;

pub open spec fn IsValidEventId(event_id: UInt32) -> bool;

pub open spec fn PlatformSupportsEvent(event_id: UInt32) -> bool;

pub open spec fn IsReservedEventAttrId(attr_id: int) -> bool;

pub open spec fn SatisfiesSharedMemoryRequirements(lo: UInt64, hi: UInt64, size: int) -> bool;

pub open spec fn EventAttrReadFailed(event_id: UInt32, base_attr_id: UInt32, attr_count: UInt32) -> bool;

pub open spec fn SharedMemoryWordAt(lo: UInt64, hi: UInt64, offset: int) -> u64;

pub open spec fn EventAttrValue(event_id: UInt32, attr_id: int) -> u64;

} // verus!
