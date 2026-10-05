use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub drtm_supported: bool,
    pub tpm_locality_closed: Map<int, bool>,
    pub tpm_locality_relinquished: Map<int, bool>,
}

pub const SUCCESS: i64 = 0;
pub const NOT_SUPPORTED: i64 = -1;
pub const INVALID_PARAMETERS: i64 = -2;
pub const DENIED: i64 = -3;
pub const ALREADY_CLOSED: i64 = -4;

pub open spec fn DrtmIsSupported(s: S) -> bool;

pub open spec fn TpmLocalityIsClosed(s: S, locality: int) -> bool;

pub open spec fn TpmLocalityIsRelinquished(s: S, locality: int) -> bool;

} // verus!
