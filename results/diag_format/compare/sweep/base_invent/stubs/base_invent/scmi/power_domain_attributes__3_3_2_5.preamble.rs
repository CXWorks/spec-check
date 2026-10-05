use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;
pub type uint8 = u8;

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -4;

pub struct S {
    pub domain_id: int,
    pub power_domain_attributes__3_3_2_5_name: Seq<[uint8; 16]>,
    pub power_domain_attributes__3_3_2_5_attrs: Seq<uint32>,
}

} // verus!
