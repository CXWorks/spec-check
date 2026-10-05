use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub struct CPLi_DESC {
    pub flags: UInt32,
    pub min_power_cap: UInt32,
    pub max_power_cap: UInt32,
    pub power_cap_step: UInt32,
    pub min_cai: UInt32,
    pub max_cai: UInt32,
    pub cai_step: UInt32,
    pub cpli: Seq<UInt32>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const OUT_OF_RANGE: Int32 = -2;
pub const NOT_FOUND: Int32 = -3;
pub const result: Int32 = -100;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn PowercapDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidCpliDescIndex(s: S, domain_id: UInt32, desc_index: UInt32) -> bool;

pub open spec fn IsRequestSupported(s: S) -> bool;

pub open spec fn DomainSupportsCpc(s: S, domain_id: UInt32) -> bool;

pub open spec fn CpliDescriptor(s: S, domain_id: UInt32, desc_index: UInt32) -> CPLi_DESC;

pub open spec fn IsAscending(cpli: Seq<UInt32>) -> bool;

} // verus!
