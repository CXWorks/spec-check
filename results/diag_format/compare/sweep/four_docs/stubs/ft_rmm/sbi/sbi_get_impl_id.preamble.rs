use vstd::prelude::*;
verus! {

pub type UInt = u64;

pub struct S {
    pub dummy: u64,
}

pub open spec fn CurrentSbiImplementationId() -> UInt;

} // verus!
