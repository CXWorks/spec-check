use vstd::prelude::*;
verus! {

pub type RmiStatusCode = u64;

pub const RMI_ERROR_NOT_SUPPORTED: RmiStatusCode = 1;

pub struct S {
    pub drtm_error_code: u64,
}

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

} // verus!
