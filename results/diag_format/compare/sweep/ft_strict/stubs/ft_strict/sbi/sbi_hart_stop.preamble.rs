use vstd::prelude::*;

verus! {

pub type HartId = u64;

pub struct CallingHart {
    pub hart_id: HartId,
}

pub struct S {
    pub hart_count: u64,
}

pub open spec fn HartExecutingInSupervisorMode(s: S, hart: CallingHart) -> bool;

pub open spec fn HartOwnedBySbiImplementation(s: S, hart: CallingHart) -> bool;

pub open spec fn CallReturnsToCaller(s: S, hart: CallingHart) -> bool;

} // verus!
