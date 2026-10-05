use vstd::prelude::*;

verus! {

pub type unsigned_long = u64;

pub type UnsignedLong = u64;

pub type HartId = u64;

pub struct sbiret {
    pub code: i64,
    pub value: i64,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsAllOnes(s: S, x: u64) -> bool;

pub open spec fn IsAligned(s: S, x: u64, align: u64) -> bool;

pub open spec fn CallingHart(s: S) -> HartId;

pub open spec fn ShmemBase(s: S, hart: HartId) -> u64;

pub open spec fn ShmemSize(s: S, hart: HartId) -> u64;

pub open spec fn ShmemEnabled(s: S, hart: HartId) -> bool;

pub open spec fn Concat(s: S, hi: u64, lo: u64) -> u64;

pub open spec fn GetShmemSize(s: S) -> u64;

} // verus!
