use vstd::prelude::*;

verus! {

pub type uint32 = u32;

pub enum FfaStatusCode {
    NotSupported,
    InvalidParameters,
    NoMemory,
    Busy,
    Interrupted,
    Denied,
    Retry,
    Aborted,
    NoData,
}

pub struct S {
    pub dummy: int,
}

pub spec const FFA_ID_GET: uint32 = 0x84000069u32;

pub spec const FFA_SUCCESS: Result<(), FfaStatusCode> = Ok(());

pub spec const NOT_SUPPORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::NotSupported);

pub open spec fn IsImplementedAtInstance(s: S, func_id: uint32) -> bool;

pub open spec fn ResultEqual(a: Result<(), FfaStatusCode>, b: Result<(), FfaStatusCode>) -> bool;

pub open spec fn CallerId() -> uint32;

pub open spec fn IsNonSecurePhysicalInstance() -> bool;

} // verus!
