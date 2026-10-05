use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: i32 = 0;

pub open spec fn IsResetSuccessful(s: S, domain_id: u32) -> bool;

} // verus!
