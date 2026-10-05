use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub enum NotSupported {
    NotSupportedErr,
    InvalidParametersErr,
    AlreadyClosedErr,
    DeniedErr,
}

pub struct S {
    pub drtm_supported: bool,
    pub locality_state: Map<u32, u8>,
}

pub spec const SUCCESS: Result<(), NotSupported> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), NotSupported> = Err(NotSupported::NotSupportedErr);
pub spec const INVALID_PARAMETERS: Result<(), NotSupported> = Err(NotSupported::InvalidParametersErr);
pub spec const ALREADY_CLOSED: Result<(), NotSupported> = Err(NotSupported::AlreadyClosedErr);
pub spec const DENIED: Result<(), NotSupported> = Err(NotSupported::DeniedErr);

pub open spec fn DrtmIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(a: Result<(), NotSupported>, b: Result<(), NotSupported>) -> bool;

pub open spec fn TpmLocalityIsClosed(s: S, locality: UInt32) -> bool;

pub open spec fn TpmLocalityIsRelinquished(s: S, locality: UInt32) -> bool;

} // verus!
