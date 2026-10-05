use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub type UInt64 = u64;

pub struct S {
    pub drtm_error: u64,
    pub dce_stage_completed: bool,
    pub drtm_supported: bool,
}

pub const SUCCESS: Int64 = 0;

pub const NOT_SUPPORTED: Int64 = -1;

pub const INVALID_PARAMETERS: Int64 = -2;

pub const DENIED: Int64 = -3;

pub const DRTM_ERROR_PHASE_DLME: UInt64 = 3;

#[allow(non_upper_case_globals)]
pub const error_code: UInt64 = 0x1234;

pub open spec fn DrtmIsSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn DrtmErrorPreviouslySet() -> bool;

pub open spec fn PersistedInNonVolatileSecureStorage(v: UInt64) -> bool;

pub open spec fn PersistedDrtmError() -> UInt64;

pub open spec fn DceStageCompleted() -> bool;

pub open spec fn Bits(v: UInt64, hi: int, lo: int) -> UInt64;

} // verus!
