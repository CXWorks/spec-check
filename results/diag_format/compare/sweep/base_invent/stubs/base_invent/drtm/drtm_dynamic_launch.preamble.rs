use vstd::prelude::*;
verus! {

pub type RmiStatusCode = u64;

pub const RMI_SUCCESS: RmiStatusCode = 0;
pub const RMI_ERROR_INPUT: RmiStatusCode = 1;

pub struct S {
    pub dummy: u64,
}

pub open spec fn R65000(s: S) -> bool;

pub open spec fn R65010(s: S) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

} // verus!
