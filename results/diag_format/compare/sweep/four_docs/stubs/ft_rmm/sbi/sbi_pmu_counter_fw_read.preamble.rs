use vstd::prelude::*;
verus! {

#[allow(non_camel_case_types)]
pub type long = i64;

#[allow(non_camel_case_types)]
pub type unsigned_long = u64;

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: long = 0;
pub const SBI_ERR_INVALID_PARAM: long = -3;

pub open spec fn IsValidCounter(s: S, counter_idx: unsigned_long) -> bool;

pub open spec fn IsHardwareCounter(s: S, counter_idx: unsigned_long) -> bool;

pub open spec fn ResultEqual(error: long, code: long) -> bool;

pub open spec fn FirmwareCounterValue(s: S, counter_idx: unsigned_long) -> unsigned_long;

pub open spec fn IsRV32(s: S) -> bool;

} // verus!
