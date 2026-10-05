use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub struct S {
    pub dummy: int,
}

pub struct PrivateEventStateT {
    pub dummy: int,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const DENIED: Int64 = -3;

pub open spec fn old<T>(x: T) -> T;

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn AnySharedEventHandlerRunning() -> bool;

pub open spec fn AnyInterruptEventBindingRegistered() -> bool;

pub open spec fn AnySharedEventRegistered() -> bool;

pub open spec fn AnyInterruptBoundToEvent() -> bool;

pub open spec fn SharedEventAuxInfoCleared() -> bool;

pub open spec fn InterruptBindingAuxInfoCleared() -> bool;

pub open spec fn PrivateEventState() -> PrivateEventStateT;

} // verus!
