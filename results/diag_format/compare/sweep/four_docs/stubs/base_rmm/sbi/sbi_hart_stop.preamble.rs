use vstd::prelude::*;
verus! {

pub type HartId = u64;
pub type OwnerId = u64;

pub struct sbiret {
    pub ret: i64,
    pub value: i64,
}

pub struct S {
    pub dummy: int,
}

pub spec const calling_hart: HartId = 0;

pub spec const SBI_IMPLEMENTATION: OwnerId = 1;

pub open spec fn HartExecutingInSMode(s: S, hart: HartId) -> bool;

pub open spec fn HartOwner(s: S, hart: HartId) -> OwnerId;

pub open spec fn CallReturns() -> bool;

} // verus!
