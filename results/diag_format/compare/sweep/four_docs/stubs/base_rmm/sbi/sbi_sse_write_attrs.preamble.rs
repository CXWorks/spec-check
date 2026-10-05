use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int64 = i64;

pub struct sbiret {
    pub error: Int64,
    pub value: UInt64,
}

pub struct S {
    pub event_attrs: Map<(UInt32, int), UInt64>,
    pub shared_memory: Map<int, UInt64>,
}

pub const SBI_SUCCESS: Int64 = 0;
pub const SBI_ERR_FAILED: Int64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: Int64 = -2;
pub const SBI_ERR_INVALID_PARAM: Int64 = -3;
pub const SBI_ERR_DENIED: Int64 = -4;
pub const SBI_ERR_INVALID_ADDRESS: Int64 = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: Int64 = -6;
pub const SBI_ERR_ALREADY_STARTED: Int64 = -7;
pub const SBI_ERR_ALREADY_STOPPED: Int64 = -8;
pub const SBI_ERR_NO_SHMEM: Int64 = -9;
pub const SBI_ERR_INVALID_STATE: Int64 = -10;
pub const SBI_ERR_BAD_RANGE: Int64 = -11;

pub const XLEN: UInt32 = 64;

pub open spec fn IsValidEventId(event_id: UInt32) -> bool;

pub open spec fn IsEventSupportedByPlatform(event_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: sbiret, code: Int64) -> bool;

pub open spec fn InputAttrValue(i: UInt32) -> UInt64;

pub open spec fn IsLegalAttrValue(event_id: UInt32, attr_id: int, value: UInt64) -> bool;

pub open spec fn IsReadOnlyAttr(event_id: UInt32, attr_id: int) -> bool;

pub open spec fn AttrValueSatisfiesStateRules(event_id: UInt32, attr_id: int, value: UInt64) -> bool;

pub open spec fn IsReservedAttrId(attr_id: int) -> bool;

pub open spec fn IsValidSharedMemory(phys_lo: UInt64, phys_hi: UInt64, size: int) -> bool;

pub open spec fn WriteFailedForUnspecifiedReason() -> bool;

pub open spec fn EventAttr(event_id: UInt32, attr_id: int) -> UInt64;

pub open spec fn InputSharedMemoryAt(offset: int) -> UInt64;

pub open spec fn IsLocalEvent(event_id: UInt32) -> bool;

pub open spec fn IsGlobalEvent(event_id: UInt32) -> bool;

} // verus!
