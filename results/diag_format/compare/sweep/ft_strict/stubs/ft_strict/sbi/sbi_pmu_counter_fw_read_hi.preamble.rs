use vstd::prelude::*;

verus! {

pub type UInt = u64;

pub type SbiErrorCode = i64;

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub open spec fn IsHardwareCounter(s: S, counter_idx: UInt) -> bool;

pub open spec fn IsValidCounter(s: S, counter_idx: UInt) -> bool;

pub open spec fn ResultEqual(error: SbiErrorCode, code: SbiErrorCode) -> bool;

pub open spec fn Xlen() -> int;

pub open spec fn FirmwareCounterValue(s: S, counter_idx: UInt) -> UInt;

pub open spec fn Bits(v: UInt, hi: int, lo: int) -> UInt;

} // verus!
