use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub enum PsciStatusCode {
    Success,
    NotSupported,
    InvalidParameters,
    Denied,
    AlreadyOn,
    OnPending,
    InternalFailure,
    NotPresent,
    Disabled,
    InvalidAddress,
}

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Result<(), PsciStatusCode> = Ok(());

pub spec const NOT_SUPPORTED: Result<(), PsciStatusCode> = Err(PsciStatusCode::NotSupported);

pub spec const DENIED: Result<(), PsciStatusCode> = Err(PsciStatusCode::Denied);

} // verus!
