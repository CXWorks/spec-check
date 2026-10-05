use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub type PeId = u64;

pub struct SdeiEvent {
    pub event_num: u32,
}

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const DENIED: Int64 = -3;

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn IsPrivateEvent(e: SdeiEvent) -> bool;

pub open spec fn IsSharedEvent(e: SdeiEvent) -> bool;

pub open spec fn EventOwnerPe(e: SdeiEvent) -> PeId;

pub open spec fn CallingPe() -> PeId;

pub open spec fn HandlerRunning(e: SdeiEvent) -> bool;

pub open spec fn EventIsUnregistered(e: SdeiEvent) -> bool;

pub open spec fn HandlerUnregisterPending(e: SdeiEvent) -> bool;

pub open spec fn PrivateEventAuxInfoReset(pe: PeId) -> bool;

pub open spec fn EventStateUnchanged(e: SdeiEvent) -> bool;

} // verus!
