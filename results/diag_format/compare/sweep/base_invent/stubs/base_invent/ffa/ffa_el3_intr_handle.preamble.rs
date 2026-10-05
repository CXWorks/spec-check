use vstd::prelude::*;
verus! {

pub type FfaReturnCode = i32;

pub const FFA_SUCCESS: FfaReturnCode = 0;
pub const FFA_ERROR_NOT_SUPPORTED: FfaReturnCode = -1;

pub struct S {
    pub dummy: u64,
}

pub open spec fn FFAInstanceIsUnsupported(s: S) -> bool;

pub open spec fn ScrEl3FiqIsSet(s: S) -> bool;

} // verus!
