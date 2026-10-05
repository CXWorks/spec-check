use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub type UInt64 = u64;

pub struct S {
    pub drtm_error: u64,
    pub dlme_phase: bool,
    pub drtm_supported: bool,
}

pub const NOT_SUPPORTED: Int64 = -1;

pub const DENIED: Int64 = -2;

pub const SUCCESS: Int64 = 0;

pub const DLME: u64 = 3;

pub const error_code: u64 = 0x1234_5678_9ABC_DEF0;

pub open spec fn IsDrtmSupported() -> bool;

pub open spec fn IsDrtmErrorSet() -> bool;

pub open spec fn IsDlmePhase() -> bool;

pub open spec fn StoredDrtmError() -> u64;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

} // verus!
