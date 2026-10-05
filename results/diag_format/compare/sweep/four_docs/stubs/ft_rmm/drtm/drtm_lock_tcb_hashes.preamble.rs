use vstd::prelude::*;
verus! {

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(self) -> bool;
}

pub struct NotSupported {
    pub code: u32,
}

pub type ReturnCode = u32;

pub const SUCCESS: ReturnCode = 0;
pub const NOT_SUPPORTED: ReturnCode = 1;
pub const DENIED: ReturnCode = 2;

pub struct S {
    pub drtm_supported: bool,
    pub tcb_hashes_locked: bool,
}

pub open spec fn IsDrtmSupported(s: S) -> bool;

pub open spec fn TcbHashesLocked(s: S) -> bool;

pub open spec fn ResultEqual(result: Result<(), NotSupported>, code: ReturnCode) -> bool;

} // verus!
