use vstd::prelude::*;
verus! {

pub type unsigned_long = u64;
pub type long = i64;

pub struct S {
    pub dummy: int,
}

pub spec const SBI_SUCCESS: long = 0;
pub spec const SBI_ERR_FAILED: long = -1;
pub spec const SBI_ERR_INVALID_PARAM: long = -3;
pub spec const SBI_ERR_INVALID_ADDRESS: long = -5;

#[allow(non_upper_case_globals)]
pub spec const result: long = -1000;

pub open spec fn ResultEqual(a: long, b: long) -> bool;

pub open spec fn IsDisableRequest(s: S, lo: unsigned_long, hi: unsigned_long) -> bool;

pub open spec fn XLEN(s: S) -> unsigned_long;

pub open spec fn TrigMax(s: S) -> unsigned_long;

pub open spec fn PhysAddr(s: S, hi: unsigned_long, lo: unsigned_long) -> int;

pub open spec fn ShmemSatisfiesSection3_2Requirements(s: S, addr: int, size: int) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason(s: S) -> bool;

pub open spec fn CallingHart(s: S) -> unsigned_long;

pub open spec fn DebugShmemEnabled(s: S, hart: unsigned_long) -> bool;

pub open spec fn DebugShmemBase(s: S, hart: unsigned_long) -> int;

pub open spec fn DebugShmemSize(s: S, hart: unsigned_long) -> int;

} // verus!
