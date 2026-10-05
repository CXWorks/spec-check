use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u64,
}

pub open spec fn PsciStatNodeStateIsValid(s: S, target_cpu: u64, power_state: u32) -> bool;

} // verus!
