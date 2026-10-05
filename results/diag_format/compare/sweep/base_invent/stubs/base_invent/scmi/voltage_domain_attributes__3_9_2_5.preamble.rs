use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;
pub type uint8 = u8;

pub struct S {
    pub voltage_domain_count: uint32,
    pub domain_attributes: Seq<uint32>,
    pub domain_names: Seq<Seq<uint8>>,
}

} // verus!
