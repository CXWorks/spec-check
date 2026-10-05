use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub enum DrtmStatusCode {
    NotSupported,
    Denied,
    InvalidParameters,
}

pub struct S {
    pub drtm_supported: bool,
    pub drtm_error_set: bool,
    pub dlme_phase: bool,
    pub stored_drtm_error: Int64,
}

pub spec const SUCCESS: Result<(), DrtmStatusCode> = Ok(());

pub spec const NOT_SUPPORTED: Result<(), DrtmStatusCode> = Err(DrtmStatusCode::NotSupported);

pub spec const DENIED: Result<(), DrtmStatusCode> = Err(DrtmStatusCode::Denied);

pub spec const DLME: Int64 = 1;

pub open spec fn IsDrtmSupported(s: S) -> bool;

pub open spec fn IsDrtmErrorSet(s: S) -> bool;

pub open spec fn IsDlmePhase(s: S) -> bool;

pub open spec fn StoredDrtmError(s: S) -> Int64;

pub open spec fn ResultEqual(a: Result<(), DrtmStatusCode>, b: Result<(), DrtmStatusCode>) -> bool;

} // verus!
