use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type SbiError = i64;

pub struct SbiRet {
    pub error: SbiError,
    pub value: UInt64,
}

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: SbiError = 0;
pub const SBI_ERR_NOT_SUPPORTED: SbiError = -2;
pub const SBI_ERR_INVALID_PARAM: SbiError = -3;
pub const SBI_ERR_DENIED: SbiError = -4;
pub const SBI_ERR_INVALID_ADDRESS: SbiError = -5;
pub const SBI_ERR_INVALID_STATE: SbiError = -10;
pub const SBI_ERR_BAD_RANGE: SbiError = -11;

pub open spec fn SbiXlenBytes(s: S) -> nat;
pub open spec fn SseEventIdIsValid(s: S, event_id: UInt32) -> bool;
pub open spec fn SseEventIsSupported(s: S, event_id: UInt32) -> bool;
pub open spec fn SseAttrIdIsReserved(attr_id: int) -> bool;
pub open spec fn SbiSharedMemIsValid(s: S, phys_lo: UInt64, phys_hi: UInt64, size: int) -> bool;
pub open spec fn SbiSharedMemReadXlen(s: S, phys_lo: UInt64, phys_hi: UInt64, offset: int) -> UInt64;
pub open spec fn SseAttrValueIsLegal(s: S, event_id: UInt32, attr_id: int, value: UInt64) -> bool;
pub open spec fn SseAttrIsReadOnly(s: S, event_id: UInt32, attr_id: int) -> bool;
pub open spec fn SseAttrValueSatisfiesStateRules(s: S, event_id: UInt32, attr_id: int, value: UInt64) -> bool;
pub open spec fn SseEventIsGlobal(s: S, event_id: UInt32) -> bool;
pub open spec fn SbiHartIsValid(s: S, hart: int) -> bool;
pub open spec fn SseEventAttr(s: S, hart: int, event_id: UInt32, attr_id: int) -> UInt64;
pub open spec fn SbiCurrentHart(s: S) -> int;

} // verus!
