use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt64 = u64;

pub enum SdeiCommandReturnCode {
    Success,
    NotSupported,
    InvalidParameters,
    Denied,
    Pending,
    OutOfResource,
}

pub struct S {
    pub sdei_supported: bool,
    pub pending_events: Map<(UInt64, Int32), bool>,
}

pub spec const SUCCESS: SdeiCommandReturnCode = SdeiCommandReturnCode::Success;
pub spec const NOT_SUPPORTED: SdeiCommandReturnCode = SdeiCommandReturnCode::NotSupported;
pub spec const INVALID_PARAMETERS: SdeiCommandReturnCode = SdeiCommandReturnCode::InvalidParameters;

pub spec const SDEI_SUCCESS: Result<(), SdeiCommandReturnCode> = Ok(());

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn ResultEqual(result: Result<(), SdeiCommandReturnCode>, code: SdeiCommandReturnCode) -> bool;
pub open spec fn IsValidMpidr(s: S, mpidr: UInt64) -> bool;
pub open spec fn EventIsPending(s: S, target_pe: UInt64, event: Int32) -> bool;
pub open spec fn EventIsPrivateTo(s: S, target_pe: UInt64, event: Int32) -> bool;
pub open spec fn ClientSharedDataObservableBeforeEvent(s: S, target_pe: UInt64, event: Int32) -> bool;

} // verus!
