use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int64 = i64;
pub type PeId = u64;

pub struct S {
    pub sdei_supported: bool,
    pub calling_pe: PeId,
}

pub const SDEI_SUCCESS: Int64 = 0;
pub const SDEI_NOT_SUPPORTED: Int64 = -1;
pub const SDEI_INVALID_PARAMETERS: Int64 = -2;
pub const SDEI_DENIED: Int64 = -3;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn CallingPe(s: S) -> PeId;

pub open spec fn AnyPrivateEventHandlerRunning(s: S, pe: PeId) -> bool;

pub open spec fn AllPrivateEventsUnregistered(s: S, pe: PeId) -> bool;

pub open spec fn PrivateEventAuxInfoWarmBootState(s: S, pe: PeId) -> bool;

pub open spec fn SharedEventsUnchanged(old_s: S, new_s: S) -> bool;

pub open spec fn SdeiStateUnchanged(old_s: S, new_s: S) -> bool;

} // verus!
