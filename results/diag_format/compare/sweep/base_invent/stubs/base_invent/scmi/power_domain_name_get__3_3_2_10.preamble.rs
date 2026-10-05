use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;
pub type uint8 = u8;

pub struct PowerDomain {
    pub id: uint32,
    pub name: [uint8; 64],
}

pub struct S {
    pub power_domains: Seq<Option<PowerDomain>>,
    pub domain_id: uint32,
    pub flags: uint32,
    pub ext_name: [uint8; 64],
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -1;

} // verus!
