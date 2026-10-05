use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct S {
    pub dummy: u64,
}

pub open spec fn CurrentSbiImplementationId() -> UInt64;

} // verus!
