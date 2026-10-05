use vstd::prelude::*;

verus! {

pub type Int = int;

pub type HartId = int;

pub struct S {
    pub dummy: int,
}

pub spec const calling_hart: HartId = 0;

pub open spec fn HartExecutingInSupervisorMode(s: S, hart: HartId) -> bool;

pub open spec fn HartOwnedBySbiImplementation(s: S, hart: HartId) -> bool;

pub open spec fn CallReturnsToCaller(s: S, hart: HartId) -> bool;

} // verus!
