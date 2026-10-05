use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

#[allow(non_camel_case_types)]
pub type int64 = i64;

pub enum SdeIStatusCode {
    NotSupported,
    InvalidParameters,
    Denied,
    OutOfResource,
}

pub struct S {
    pub dummy: int,
}

pub spec const SDEI_SUCCESS: Result<int64, SdeIStatusCode> = Ok(0i64);

pub spec const SDEI_ERROR_NOT_SUPPORTED: Result<int64, SdeIStatusCode> = Err(SdeIStatusCode::NotSupported);

pub spec const SDEI_ERROR_INVALID_PARAMETERS: Result<int64, SdeIStatusCode> = Err(SdeIStatusCode::InvalidParameters);

pub spec const SDEI_ERROR_DENIED: Result<int64, SdeIStatusCode> = Err(SdeIStatusCode::Denied);

pub spec const SDEI_ERROR_OUT_OF_RESOURCE: Result<int64, SdeIStatusCode> = Err(SdeIStatusCode::OutOfResource);

} // verus!
