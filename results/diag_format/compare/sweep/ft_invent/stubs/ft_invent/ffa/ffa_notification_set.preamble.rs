use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub enum FfaStatusCode {
    InvalidParameters,
    NotSupported,
    Denied,
    Aborted,
    Busy,
    Retry,
    NoMemory,
}

pub struct S {
    pub per_cpu_notifications_supported: bool,
    pub notifications_bound: u32,
    pub notifications_pending: u32,
}

pub spec const FFA_SUCCESS: Result<(), FfaStatusCode> = Ok(());
pub spec const FFA_ERROR_INVALID_PARAMETERS: Result<(), FfaStatusCode> = Err(FfaStatusCode::InvalidParameters);
pub spec const FFA_ERROR_NOT_SUPPORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::NotSupported);
pub spec const FFA_ERROR_DENIED: Result<(), FfaStatusCode> = Err(FfaStatusCode::Denied);
pub spec const FFA_ERROR_ABORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::Aborted);

pub open spec fn PerCpuNotificationsSupported(s: S) -> bool;

} // verus!
