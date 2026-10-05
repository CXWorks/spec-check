use vstd::prelude::*;

verus! {

pub type UInt = u64;

pub type SbiErrorCode = i64;

pub struct S {
    pub counter_idx_reg: UInt,
    pub num_hw_counters: UInt,
    pub num_counters: UInt,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;

pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub open spec fn counter_idx(s: S) -> UInt;

pub open spec fn IsHardwareCounter(s: S, idx: UInt) -> bool;

pub open spec fn IsValidCounter(s: S, idx: UInt) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn Xlen() -> int;

pub open spec fn FirmwareCounterValue(idx: UInt) -> UInt;

pub open spec fn Bits(x: UInt, hi: int, lo: int) -> UInt;

} // verus!
