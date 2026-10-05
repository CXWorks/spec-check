use vstd::prelude::*;

verus! {

pub struct S {
    pub sdei_implemented: bool,
    pub calling_pe: nat,
    pub pe_masked: Map<nat, bool>,
    pub pending_events: Map<nat, Seq<nat>>,
}

pub const SUCCESS: i64 = 0;
pub const NOT_SUPPORTED: i64 = -1;

pub open spec fn SdeiIsImplemented(s: S) -> bool;

pub open spec fn SdeiCallingPeIsMasked(s: S) -> bool;

pub open spec fn SdeiOtherPesMaskUnchanged(old_s: S, new_s: S) -> bool;

pub open spec fn SdeiPendingEventsDispatchedForCallingPe(old_s: S, new_s: S) -> bool;

} // verus!
