use vstd::prelude::*;
verus! {

pub type Int64 = i64;

pub struct S {
    pub sdei_supported: bool,
    pub shared_event_state: Seq<u64>,
    pub private_event_state: Seq<u64>,
    pub interrupt_bindings: Seq<u64>,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const DENIED: Int64 = -3;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn SdeiAnySharedEventHandlerRunning(s: S) -> bool;
pub open spec fn SdeiAnyInterruptEventBindingRegistered(s: S) -> bool;
pub open spec fn SdeiNoSharedEventRegistered(s: S) -> bool;
pub open spec fn SdeiSharedEventAuxInfoCleared(s: S) -> bool;
pub open spec fn SdeiPrivateEventsUnchanged(old_s: S, new_s: S) -> bool;

} // verus!
