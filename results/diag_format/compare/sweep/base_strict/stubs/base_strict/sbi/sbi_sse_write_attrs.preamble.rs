use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type UInt = u64;

#[allow(non_upper_case_globals)]
pub const UInt: u64 = 8;

pub type SbiError = i64;

pub const SBI_SUCCESS: SbiError = 0;
pub const SBI_ERR_FAILED: SbiError = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiError = -2;
pub const SBI_ERR_INVALID_PARAM: SbiError = -3;
pub const SBI_ERR_DENIED: SbiError = -4;
pub const SBI_ERR_INVALID_ADDRESS: SbiError = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: SbiError = -6;
pub const SBI_ERR_ALREADY_STARTED: SbiError = -7;
pub const SBI_ERR_ALREADY_STOPPED: SbiError = -8;
pub const SBI_ERR_NO_SHMEM: SbiError = -9;
pub const SBI_ERR_INVALID_STATE: SbiError = -10;
pub const SBI_ERR_BAD_RANGE: SbiError = -11;

pub struct Hart {
    pub id: u64,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn ResultEqual(a: SbiError, b: SbiError) -> bool;

pub open spec fn IsValidEventId(event_id: UInt32) -> bool;

pub open spec fn IsEventSupportedByPlatform(event_id: UInt32) -> bool;

pub open spec fn IsReservedAttrId(attr_id: int) -> bool;

pub open spec fn SharedMemoryMeetsRequirements(lo: UInt, hi: UInt, size: int) -> bool;

pub open spec fn IsReadOnlyAttr(event_id: UInt32, attr_id: int) -> bool;

pub open spec fn InputValueAt(lo: UInt, hi: UInt, offset: int) -> UInt;

pub open spec fn IsLegalAttrValue(event_id: UInt32, attr_id: int, value: UInt) -> bool;

pub open spec fn AttrValueSatisfiesStateRules(event_id: UInt32, attr_id: int, value: UInt) -> bool;

pub open spec fn WriteFailedForUnspecifiedReason(event_id: UInt32, base_attr_id: UInt32, attr_count: UInt32) -> bool;

pub open spec fn IsLocalEvent(event_id: UInt32) -> bool;

pub open spec fn IsGlobalEvent(event_id: UInt32) -> bool;

pub open spec fn CallingHart() -> Hart;

pub open spec fn AttrValue(event_id: UInt32, hart: Hart, attr_id: int) -> UInt;

} // verus!
