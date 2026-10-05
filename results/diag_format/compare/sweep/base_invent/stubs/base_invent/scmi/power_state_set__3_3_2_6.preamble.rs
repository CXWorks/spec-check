use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub struct S {
    pub flags: u32,
    pub domain_id: u32,
    pub power_state: u32,
}

} // verus!
