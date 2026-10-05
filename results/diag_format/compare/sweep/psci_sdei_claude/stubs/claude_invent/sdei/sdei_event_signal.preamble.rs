use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub const NOT_SUPPORTED: i64 = -1;
pub const INVALID_PARAMETERS: i64 = -2;
pub const SUCCESS: i64 = 0;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn SdeiIsValidTargetPe(s: S, target_pe: u64) -> bool;
pub open spec fn SdeiEventIsPending(s: S, target_pe: u64, event: int) -> bool;

} // verus!
