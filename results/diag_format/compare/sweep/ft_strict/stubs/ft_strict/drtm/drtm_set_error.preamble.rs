use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub struct S {
    pub dummy: int,
}

pub enum DrtmStatusCode {
    NotSupported,
    Denied,
    InvalidParameters,
    InternalError,
}

pub spec const SUCCESS: Result<(), DrtmStatusCode> = Ok(());

pub spec const NOT_SUPPORTED: Result<(), DrtmStatusCode> = Err(DrtmStatusCode::NotSupported);

pub spec const DENIED: Result<(), DrtmStatusCode> = Err(DrtmStatusCode::Denied);

pub spec const DRTM_ERROR_PHASE_DLME: Int64 = 3;

pub open spec fn DrtmIsSupported(s: S) -> bool;

pub open spec fn DrtmErrorPreviouslySet(s: S) -> bool;

pub open spec fn ResultEqual(a: Result<(), DrtmStatusCode>, b: Result<(), DrtmStatusCode>) -> bool;

pub open spec fn PersistedInNonVolatileSecureStorage(s: S, v: Int64) -> bool;

pub open spec fn PersistedDrtmError(s: S) -> Int64;

pub open spec fn DceStageCompleted(s: S) -> bool;

pub open spec fn Bits(v: Int64, hi: int, lo: int) -> Int64;

} // verus!
