use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub num_reset_domains: u32,
}

pub open spec fn NumResetDomains(s: S) -> int;

} // verus!
