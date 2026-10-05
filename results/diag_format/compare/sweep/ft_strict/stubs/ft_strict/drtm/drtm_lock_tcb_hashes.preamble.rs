use vstd::prelude::*;
verus! {

pub struct NotSupported {}

pub struct S {
    pub drtm_supported: bool,
    pub tcb_hashes_locked: bool,
}

pub type ReturnCode = i64;

pub const SUCCESS: ReturnCode = 0;
pub const NOT_SUPPORTED: ReturnCode = -1;
pub const DENIED: ReturnCode = -3;

pub open spec fn DrtmIsSupported(s: S) -> bool;

pub open spec fn TcbHashesLocked(s: S) -> bool;

pub open spec fn ResultEqual(result: Result<(), NotSupported>, code: ReturnCode) -> bool;

} // verus!
