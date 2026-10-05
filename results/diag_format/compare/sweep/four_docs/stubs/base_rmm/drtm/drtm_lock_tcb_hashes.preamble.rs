use vstd::prelude::*;
verus! {

pub type Int64 = i64;

pub struct S {
    pub tcb_hashes_locked: bool,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const DENIED: Int64 = -3;

pub open spec fn IsDrtmSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn TcbHashesLocked(s: S) -> bool;

} // verus!
