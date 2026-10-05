use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type UInt64 = u64;
pub type EventId = u64;
pub type PeId = u64;

pub struct S {
    pub dummy: u64,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const DENIED: Int64 = -3;

pub spec const event: EventId = 0;

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn CallingPe() -> PeId;

pub open spec fn HandlerRunning(pe: PeId) -> bool;

pub open spec fn IsPrivateEvent(e: EventId) -> bool;

pub open spec fn IsSharedEvent(e: EventId) -> bool;

pub open spec fn EventHandlingComplete(e: EventId, pe: PeId) -> bool;

pub open spec fn EventHandlingCompleteGlobally(e: EventId) -> bool;

} // verus!
