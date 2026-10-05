use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub open spec fn CurrentHartId(s: S) -> u64;

pub open spec fn HartResumedFromStopped(old_s: S, new_s: S, hart_id: u64) -> bool;

pub open spec fn IsSupervisorMode(s: S) -> bool;

pub open spec fn HartPc(s: S) -> u64;

pub open spec fn HartSatp(s: S) -> u64;

pub open spec fn HartSstatusSie(s: S) -> u64;

pub open spec fn HartRegA0(s: S) -> u64;

pub open spec fn HartRegA1(s: S) -> u64;

} // verus!
