use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub type UInt32 = u32;

pub struct S {
    pub tpm_locality_states: Seq<u64>,
}

pub const locality: UInt32 = 2;

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;
pub const ALREADY_CLOSED: Int64 = -4;

pub open spec fn DrtmIsSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn LocalityIsClosed(l: UInt32) -> bool;

pub open spec fn LocalityIsRelinquished(l: UInt32) -> bool;

pub open spec fn TpmLocalityState(s: S, l: UInt32) -> u64;

} // verus!
