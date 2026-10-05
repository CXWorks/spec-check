use vstd::prelude::*;

verus! {

pub struct S {
    pub dummy: u64,
}

pub open spec fn AllEnabledDesCollectedViaShmtiOrFastChannels(s: S) -> bool;

} // verus!
