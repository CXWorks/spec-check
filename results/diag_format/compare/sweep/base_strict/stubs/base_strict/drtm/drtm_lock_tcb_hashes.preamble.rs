use vstd::prelude::*;
verus! {

pub type Int64 = i64;

pub struct S {
    pub drtm_supported: bool,
    pub tcb_hashes_locked: bool,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const DENIED: Int64 = -3;

pub open spec fn DrtmIsSupported(s: S) -> bool;

pub open spec fn TcbHashesLocked(s: S) -> bool;

pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;

} // verus!
