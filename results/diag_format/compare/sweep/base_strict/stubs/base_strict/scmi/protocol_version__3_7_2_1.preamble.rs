use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type RmiStatusCode = u64;

pub spec const SUCCESS: RmiStatusCode = 0;

pub struct S {
    pub dummy: u64,
}

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

} // verus!
