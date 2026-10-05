use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int64 = i64;

pub struct S {
    pub dummy: int,
}

pub struct PrivateEvents {
    pub data: int,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const DENIED: Int64 = -3;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;

pub open spec fn AnySharedEventHandlerRunning(s: S) -> bool;

pub open spec fn AnyInterruptEventBindingRegistered(s: S) -> bool;

pub open spec fn AnySharedEventRegistered(s: S) -> bool;

pub open spec fn AnyInterruptBoundToEvent(s: S) -> bool;

pub open spec fn SharedEventAuxInfoCleared(s: S) -> bool;

pub open spec fn InterruptBindingAuxInfoCleared(s: S) -> bool;

pub open spec fn PrivateEventState(s: S) -> PrivateEvents;

} // verus!
