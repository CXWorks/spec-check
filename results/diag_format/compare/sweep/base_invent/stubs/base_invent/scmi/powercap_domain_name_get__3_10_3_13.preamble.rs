use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;
pub type uint8 = u8;

pub struct S {
    pub powercap_domains: Map<uint32, uint32>,
    pub domain_id: uint32,
}

} // verus!
