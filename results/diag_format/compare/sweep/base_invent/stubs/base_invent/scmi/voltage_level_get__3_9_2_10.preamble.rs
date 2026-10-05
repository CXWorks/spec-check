use vstd::prelude::*;
verus! {

pub type int32 = int;

pub const domain_id: u32 = 7;

pub struct S {
    pub dummy: int,
}

impl S {
    pub open spec fn voltage_level_get(self, s: S, domain: int) -> int32;
}

} // verus!
