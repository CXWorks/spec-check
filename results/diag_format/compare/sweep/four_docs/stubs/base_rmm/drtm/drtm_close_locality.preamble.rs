use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int64 = 0;
pub spec const NOT_SUPPORTED: Int64 = (-1int) as Int64;
pub spec const INVALID_PARAMETERS: Int64 = (-2int) as Int64;
pub spec const DENIED: Int64 = (-3int) as Int64;
pub spec const ALREADY_CLOSED: Int64 = (-4int) as Int64;

pub open spec fn DrtmIsSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn TpmLocalityIsClosed(s: S, locality: UInt32) -> bool;

pub open spec fn TpmLocalityIsRelinquished(s: S, locality: UInt32) -> bool;

} // verus!
