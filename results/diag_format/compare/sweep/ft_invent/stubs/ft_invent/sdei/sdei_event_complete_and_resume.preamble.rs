use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type PeId = u64;

pub enum SdeiStatusCode {
    InvalidParameters,
    Denied,
    NotSupported,
    Pending,
    OutOfResource,
}

pub struct S {
    pub dummy: u64,
}

pub spec const SDEI_SUCCESS: Result<(), SdeiStatusCode> = Ok(());

pub spec const SDEI_ERROR_INVALID_PARAMETERS: Result<(), SdeiStatusCode> = Err(SdeiStatusCode::InvalidParameters);

pub spec const SDEI_ERROR_DENIED: Result<(), SdeiStatusCode> = Err(SdeiStatusCode::Denied);

pub open spec fn HandlerRunning(s: S, pe: PeId) -> bool;

pub open spec fn current_pe() -> PeId;

} // verus!
