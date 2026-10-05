use vstd::prelude::*;
verus! {

pub type Int64 = i64;

pub type PeIndex = u64;

pub type EventNumber = u32;

pub struct S {
    pub dummy: int,
}

pub spec const NOT_SUPPORTED: Int64 = -1;
pub spec const DENIED: Int64 = -3;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn CallingPe() -> PeIndex;

pub open spec fn HandlerRunning(s: S, pe: PeIndex) -> bool;

pub open spec fn CallReturns(s: S, pe: PeIndex) -> bool;

pub open spec fn ExecutionResumesAtInterruptedContext(s: S, pe: PeIndex) -> bool;

pub open spec fn RunningEvent(s: S, pe: PeIndex) -> EventNumber;

pub open spec fn IsPrivateEvent(s: S, ev: EventNumber) -> bool;

pub open spec fn IsSharedEvent(s: S, ev: EventNumber) -> bool;

pub open spec fn EventHandlingCompleteForPe(s: S, ev: EventNumber, pe: PeIndex) -> bool;

pub open spec fn EventHandlingCompleteGlobally(s: S, ev: EventNumber) -> bool;

} // verus!
