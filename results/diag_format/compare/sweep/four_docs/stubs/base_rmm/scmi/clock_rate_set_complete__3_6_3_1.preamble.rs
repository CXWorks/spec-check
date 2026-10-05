use vstd::prelude::*;
verus! {

pub type RmiStatusCode = u64;

pub spec const SUCCESS: RmiStatusCode = 0;
pub spec const DENIED: RmiStatusCode = 1;

pub type ClockId = u64;

pub spec const clock_id: ClockId = 0;

pub spec const rate: Seq<u32> = seq![0u32, 0u32];

pub struct Clock {
    pub rate: int,
}

pub struct S {
    pub clocks: Map<ClockId, Clock>,
}

pub open spec fn ClockHasOtherUsers(s: S, id: ClockId) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn ClockAt(s: S, id: ClockId) -> Clock;

} // verus!
