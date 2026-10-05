use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = -5;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsXlenAllOnes(s: S, v: UInt64) -> bool;

pub open spec fn Xlen(s: S) -> UInt64;

pub open spec fn DebugTrigMax(s: S) -> UInt64;

pub open spec fn SbiShmemPhysAddr(s: S, lo: UInt64, hi: UInt64) -> int;

pub open spec fn SbiShmemAddressValid(s: S, addr: int, size: int) -> bool;

pub open spec fn CurrentHart(s: S) -> int;

pub open spec fn HartDebugShmemEnabled(s: S, hart: int) -> bool;

pub open spec fn HartDebugShmemBase(s: S, hart: int) -> int;

} // verus!
