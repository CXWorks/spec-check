use vstd::prelude::*;

verus! {

pub type Bits64 = u64;

pub type Int64 = i64;

pub struct S {
    pub drtm_supported: bool,
    pub error_set: bool,
    pub stored_error: u64,
    pub dce_phase_completed: bool,
}

pub const SUCCESS: Int64 = 0;

pub const NOT_SUPPORTED: Int64 = -1;

pub const DENIED: Int64 = -3;

pub open spec fn DrtmIsSupported(s: S) -> bool;

pub open spec fn DrtmErrorIsSet(s: S) -> bool;

pub open spec fn DrtmStoredError(s: S) -> Bits64;

pub open spec fn DrtmDcePhaseCompleted(s: S) -> bool;

pub open spec fn DrtmPhaseDlme() -> u64;

} // verus!
