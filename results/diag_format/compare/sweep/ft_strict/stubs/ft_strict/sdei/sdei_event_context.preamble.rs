use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int64 = i64;
pub type PeId = u64;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(self) -> bool {
        match self {
            Result::Ok(_) => true,
            Result::Err(_) => false,
        }
    }

    pub open spec fn is_Err(self) -> bool {
        match self {
            Result::Ok(_) => false,
            Result::Err(_) => true,
        }
    }
}

pub enum SdeiStatusCode {
    NotSupported,
    InvalidParameters,
    Denied,
    OutOfResource,
    Pending,
}

pub struct S {
    pub sdei_supported: bool,
    pub handler_running: Map<PeId, bool>,
    pub context_registers: Map<PeId, Seq<Int64>>,
}

pub spec const NOT_SUPPORTED: Result<Int64, SdeiStatusCode> = Result::Err(SdeiStatusCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<Int64, SdeiStatusCode> = Result::Err(SdeiStatusCode::InvalidParameters);
pub spec const DENIED: Result<Int64, SdeiStatusCode> = Result::Err(SdeiStatusCode::Denied);

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(a: Result<Int64, SdeiStatusCode>, b: Result<Int64, SdeiStatusCode>) -> bool;

pub open spec fn HandlerRunning(s: S, pe: PeId) -> bool;

pub open spec fn CallingPe() -> PeId;

pub open spec fn EventContextRegister(s: S, pe: PeId, idx: int) -> Result<Int64, SdeiStatusCode>;

} // verus!
