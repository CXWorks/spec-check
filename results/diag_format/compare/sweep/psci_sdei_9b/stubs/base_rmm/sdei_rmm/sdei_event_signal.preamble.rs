use vstd::prelude::*;
verus! {

pub type Int64 = i64;
pub type UInt64 = u64;
pub type S = u64;

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;

#[allow(non_upper_case_globals)]
pub const event: UInt64 = 1;
#[allow(non_upper_case_globals)]
pub const target_pe: UInt64 = 2;

pub open spec fn SdeiIsSupported() -> bool;
pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;
pub open spec fn IsValidMpidr(mpidr: UInt64) -> bool;
pub open spec fn EventIsPending(ev: UInt64, pe: UInt64) -> bool;
pub open spec fn EventPendingState(x: UInt64, pe: UInt64) -> bool;

} // verus!
