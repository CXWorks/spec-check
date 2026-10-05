use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt64 = u64;
pub type Int64 = i64;

pub struct S {
    pub sdei_supported: bool,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;
pub open spec fn IsValidMpidr(mpidr: UInt64) -> bool;
pub open spec fn EventIsPending(s: S, event: Int32, target_pe: UInt64) -> bool;

} // verus!
