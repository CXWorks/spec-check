use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type sbiret = i64;

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: sbiret = 0;
pub const SBI_ERR_FAILED: sbiret = -1;
pub const SBI_ERR_NOT_SUPPORTED: sbiret = -2;
pub const SBI_ERR_INVALID_PARAM: sbiret = -3;
pub const SBI_ERR_DENIED: sbiret = -4;
pub const SBI_ERR_INVALID_ADDRESS: sbiret = -5;
pub const SBI_ERR_INVALID_STATE: sbiret = -10;
pub const SBI_ERR_BAD_RANGE: sbiret = -11;

pub const XLEN: u32 = 64;

pub open spec fn IsValidEventId(s: S, event_id: UInt32) -> bool;
pub open spec fn IsEventSupportedByPlatform(s: S, event_id: UInt32) -> bool;
pub open spec fn ResultEqual(a: sbiret, b: sbiret) -> bool;
pub open spec fn Exists(b: bool) -> bool;
pub open spec fn ForAll(b: bool) -> bool;
pub open spec fn IsLegalAttrValue(s: S, event_id: UInt32, attr_id: UInt32, value: UInt64) -> bool;
pub open spec fn InputAttrValue(i: UInt32) -> UInt64;
pub open spec fn IsReadOnlyAttr(s: S, event_id: UInt32, attr_id: UInt32) -> bool;
pub open spec fn AttrValueSatisfiesStateRules(s: S, event_id: UInt32, attr_id: UInt32, value: UInt64) -> bool;
pub open spec fn IsReservedAttrId(s: S, attr_id: UInt32) -> bool;
pub open spec fn IsValidSharedMemory(s: S, lo: u64, hi: u64, size: UInt32) -> bool;
pub open spec fn WriteFailedForUnspecifiedReason() -> bool;
pub open spec fn EventAttr(s: S, event_id: UInt32, attr_id: UInt32) -> UInt64;
pub open spec fn InputSharedMemoryAt(offset: UInt32) -> UInt64;
pub open spec fn IsLocalEvent(s: S, event_id: UInt32) -> bool;
pub open spec fn IsGlobalEvent(s: S, event_id: UInt32) -> bool;

} // verus!
