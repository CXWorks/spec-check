use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type SbiError = i64;

pub const SBI_SUCCESS: SbiError = 0;
pub const SBI_ERR_FAILED: SbiError = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiError = -2;
pub const SBI_ERR_INVALID_PARAM: SbiError = -3;
pub const SBI_ERR_INVALID_ADDRESS: SbiError = -5;
pub const SBI_ERR_BAD_RANGE: SbiError = -11;

pub struct S {
    pub dummy: int,
}

pub open spec fn SseEventIdValid(s: S, event_id: UInt32) -> bool;

pub open spec fn SseEventSupported(s: S, event_id: UInt32) -> bool;

pub open spec fn SseAttrIdReserved(s: S, attr_id: int) -> bool;

pub open spec fn SseSharedMemValid(s: S, phys_lo: UInt64, phys_hi: UInt64, size: int) -> bool;

pub open spec fn SseEventStateUnchanged(old_s: S, new_s: S) -> bool;

pub open spec fn SharedMemRead64(s: S, phys_lo: UInt64, phys_hi: UInt64, offset: int) -> UInt64;

pub open spec fn SseEventAttrValue(s: S, event_id: UInt32, attr_id: int) -> UInt64;

} // verus!
