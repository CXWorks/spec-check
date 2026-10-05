use vstd::prelude::*;

verus! {

pub struct SbiRet {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

pub const SBI_SUCCESS: i64 = 0i64;
pub const SBI_ERR_INVALID_PARAM: i64 = -3i64;
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5i64;

pub open spec fn MpxyShmemEnabled(s: S, hartid: u64) -> bool;

pub open spec fn MpxyShmemBase(s: S, hartid: u64) -> u64;

pub open spec fn MpxyShmemSize(s: S) -> u64;

pub open spec fn IsValidMpxyShmemMode(s: S, mode: u64) -> bool;

pub open spec fn MpxyShmemAccessible(s: S, base: int, size: int) -> bool;

} // verus!
