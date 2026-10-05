use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt64 = u64;
pub type SdeiCommandReturnCode = i64;

pub struct S {
    pub sdei_supported: bool,
}

pub spec const SUCCESS: SdeiCommandReturnCode = 0;
pub spec const NOT_SUPPORTED: SdeiCommandReturnCode = (-1int) as i64;
pub spec const INVALID_PARAMETERS: SdeiCommandReturnCode = (-2int) as i64;

pub spec const SDEI_SUCCESS: Result<(), SdeiCommandReturnCode> = Ok(());

pub uninterp spec fn SdeiIsSupported(s: S) -> bool;
pub uninterp spec fn ResultEqual(result: Result<(), SdeiCommandReturnCode>, code: SdeiCommandReturnCode) -> bool;
pub uninterp spec fn IsValidMpidr(s: S, mpidr: UInt64) -> bool;
pub uninterp spec fn EventIsPending(s: S, event: Int32, target_pe: UInt64) -> bool;

} // verus!
