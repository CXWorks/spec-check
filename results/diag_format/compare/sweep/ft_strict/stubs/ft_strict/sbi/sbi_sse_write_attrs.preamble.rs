use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt = u64;
pub type Hart = u64;
pub type AttrVal = u64;
pub type SbiError = i64;

pub struct S {
    pub dummy: int,
}

pub const XLEN: u32 = 64;

pub const SBI_SUCCESS: SbiError = 0;
pub const SBI_ERR_FAILED: SbiError = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiError = -2;
pub const SBI_ERR_INVALID_PARAM: SbiError = -3;
pub const SBI_ERR_DENIED: SbiError = -4;
pub const SBI_ERR_INVALID_ADDRESS: SbiError = -5;
pub const SBI_ERR_BAD_RANGE: SbiError = -7;
pub const SBI_ERR_INVALID_STATE: SbiError = -10;

pub open spec fn ResultEqual(result: SbiError, code: SbiError) -> bool;
pub open spec fn IsValidEventId(s: S, event_id: UInt32) -> bool;
pub open spec fn IsEventSupportedByPlatform(s: S, event_id: UInt32) -> bool;
pub open spec fn IsReservedAttrId(s: S, attr_id: int) -> bool;
pub open spec fn SharedMemoryMeetsRequirements(s: S, lo: UInt, hi: UInt, size: int) -> bool;
pub open spec fn IsReadOnlyAttr(s: S, event_id: UInt32, attr_id: int) -> bool;
pub open spec fn IsLegalAttrValue(s: S, event_id: UInt32, attr_id: int, value: AttrVal) -> bool;
pub open spec fn AttrValueSatisfiesStateRules(s: S, event_id: UInt32, attr_id: int, value: AttrVal) -> bool;
pub open spec fn InputValueAt(s: S, lo: UInt, hi: UInt, offset: int) -> AttrVal;
pub open spec fn WriteFailedForUnspecifiedReason(s: S, event_id: UInt32, base_attr_id: UInt32, attr_count: UInt32) -> bool;
pub open spec fn IsLocalEvent(s: S, event_id: UInt32) -> bool;
pub open spec fn IsGlobalEvent(s: S, event_id: UInt32) -> bool;
pub open spec fn AttrValue(s: S, event_id: UInt32, hart: Hart, attr_id: int) -> AttrVal;
pub open spec fn CallingHart() -> Hart;

} // verus!
