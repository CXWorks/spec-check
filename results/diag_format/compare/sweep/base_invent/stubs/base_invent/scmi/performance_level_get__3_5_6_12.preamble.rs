use vstd::prelude::*;
verus! {

pub type int32 = int;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

impl S {
    pub open spec fn domain_exists(self, s: S, id: int) -> bool;
}

} // verus!
