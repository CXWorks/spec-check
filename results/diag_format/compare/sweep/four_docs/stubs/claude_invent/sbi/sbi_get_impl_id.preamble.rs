use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct S {
    pub impl_id: u64,
}

pub open spec fn SbiImplementationId(s: S) -> UInt64;

} // verus!
