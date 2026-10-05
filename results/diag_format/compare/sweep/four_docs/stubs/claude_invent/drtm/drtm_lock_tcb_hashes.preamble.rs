use vstd::prelude::*;
verus! {

pub type DrtmReturnCode = i64;

pub const SUCCESS: DrtmReturnCode = 0;
pub const NOT_SUPPORTED: DrtmReturnCode = -1;
pub const DENIED: DrtmReturnCode = -3;

pub struct S {
    pub drtm_supported: bool,
    pub tcb_hashes_locked: bool,
}

pub open spec fn DrtmIsSupported(s: S) -> bool;

pub open spec fn TcbHashesLocked(s: S) -> bool;

} // verus!
