use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type UInt8 = u8;

pub enum FfaStatusCode {
    InvalidParameters,
    NotSupported,
    Retry,
    Denied,
}

pub struct S {
    pub dummy: u64,
}

pub spec const FFA_SUCCESS: Result<(), FfaStatusCode> = Ok(());

pub spec const FFA_ERROR_INVALID_PARAMETERS: Result<(), FfaStatusCode> = Err(FfaStatusCode::InvalidParameters);

pub spec const FFA_ERROR_NOT_SUPPORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::NotSupported);

pub spec const FFA_ERROR_RETRY: Result<(), FfaStatusCode> = Err(FfaStatusCode::Retry);

pub open spec fn ResultEqual(a: Result<(), FfaStatusCode>, b: Result<(), FfaStatusCode>) -> bool;

} // verus!
