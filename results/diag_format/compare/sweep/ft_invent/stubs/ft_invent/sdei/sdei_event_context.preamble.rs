use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type int64 = i64;

pub enum SdeiStatusCode {
    InvalidParameters,
    Denied,
    NotSupported,
    Pending,
    OutOfResource,
}

pub struct SdeiEventHandler {
    pub handler_running: bool,
}

pub struct S {
    pub handlers: Seq<SdeiEventHandler>,
}

pub spec const SDEI_SUCCESS: Result<int64, SdeiStatusCode> = Ok(0i64);
pub spec const SDEI_ERROR_INVALID_PARAMETERS: Result<int64, SdeiStatusCode> = Err(SdeiStatusCode::InvalidParameters);
pub spec const SDEI_ERROR_DENIED: Result<int64, SdeiStatusCode> = Err(SdeiStatusCode::Denied);

pub open spec fn SdeiEventHandlerAt(s: S, idx: int) -> SdeiEventHandler;

} // verus!
