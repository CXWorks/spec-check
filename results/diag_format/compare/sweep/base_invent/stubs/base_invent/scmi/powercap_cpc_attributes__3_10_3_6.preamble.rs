use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;
pub type uint8 = u8;

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = 1;
pub const OUT_OF_RANGE: int32 = 2;
pub const NOT_SUPPORTED: int32 = 3;

pub struct CPLi_DESC {
    pub cpli: uint32,
    pub flags: uint32,
    pub min_power_cap: uint32,
    pub max_power_cap: uint32,
    pub min_cai: uint32,
    pub max_cai: uint32,
    pub name: [uint8; 16],
}

pub struct S {
    pub desc_index: uint32,
    pub num_cpl_descriptors: uint32,
    pub cpc_supported: bool,
}

pub open spec fn desc_index(s: S) -> uint32;

pub open spec fn num_cpl_descriptors(s: S) -> uint32;

pub open spec fn cpc_supported(s: S) -> bool;

} // verus!
