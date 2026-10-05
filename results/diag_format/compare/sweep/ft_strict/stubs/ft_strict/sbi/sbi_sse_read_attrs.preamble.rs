use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Address = int;
pub type SbiErrorCode = i64;

pub struct S {
    pub dummy: int,
}

pub spec const XLEN: int = 64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = -5;
pub const SBI_ERR_BAD_RANGE: SbiErrorCode = -11;

#[allow(non_upper_case_globals)]
pub const result: SbiErrorCode = 100;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn IsReservedEventId(s: S, event_id: UInt32) -> bool;

pub open spec fn IsValidEventId(s: S, event_id: UInt32) -> bool;

pub open spec fn PlatformSupportsEvent(s: S, event_id: UInt32) -> bool;

pub open spec fn IsReservedEventAttrId(s: S, attr_id: int) -> bool;

pub open spec fn SatisfiesSharedMemoryRequirements(s: S, lo: Address, hi: Address, size: int) -> bool;

pub open spec fn EventAttrReadFailed(s: S, event_id: UInt32, base_attr_id: UInt32, attr_count: UInt32) -> bool;

pub open spec fn SharedMemoryWordAt(s: S, lo: Address, hi: Address, offset: int) -> u64;

pub open spec fn EventAttrValue(s: S, event_id: UInt32, attr_id: int) -> u64;

} // verus!
