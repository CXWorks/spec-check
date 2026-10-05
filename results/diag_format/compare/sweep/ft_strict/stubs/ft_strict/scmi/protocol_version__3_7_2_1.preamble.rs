use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type RmiStatusCode = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const SUCCESS: RmiStatusCode = 0;

pub spec const RMI_ERROR_INPUT: RmiStatusCode = 1;

pub spec const result: Result<(), RmiStatusCode> = Ok(());

pub open spec fn ResultEqual(r: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

} // verus!
