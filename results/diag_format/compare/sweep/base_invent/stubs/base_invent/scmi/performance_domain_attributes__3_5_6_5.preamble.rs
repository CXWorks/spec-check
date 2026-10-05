use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -3;

pub const domain_id: u32 = 0;

pub struct S {
    pub attributes: u32,
    pub rate_limit: u32,
    pub sustained_freq: u32,
    pub sustained_perf_level: u32,
    pub name: Seq<u8>,
    pub guaranteed_perf_level: u32,
    pub qos_capability_types: u32,
    pub qos_parent_id: u32,
}

pub open spec fn DomainExists(s: S, id: u32) -> bool;

} // verus!
