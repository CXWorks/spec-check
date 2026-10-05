use vstd::prelude::*;
verus! {

pub type CallingHart = u64;

pub type HartOwnerId = u64;

pub const SBI_IMPLEMENTATION: HartOwnerId = 1;

pub struct S {
    pub dummy: u64,
}

pub open spec fn HartExecutingInSMode(s: S, hart: CallingHart) -> bool;

pub open spec fn HartOwner(s: S, hart: CallingHart) -> HartOwnerId;

pub open spec fn CallReturns(s: S) -> bool;

} // verus!
