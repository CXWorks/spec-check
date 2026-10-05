use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub struct S {
    pub num_reset_domains: u32,
}

pub open spec fn StatusIsSuccess(status: Int32) -> bool;

pub open spec fn NumResetDomains() -> UInt32;

} // verus!
