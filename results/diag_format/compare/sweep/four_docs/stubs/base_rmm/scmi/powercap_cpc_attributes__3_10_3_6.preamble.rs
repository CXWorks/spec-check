use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt16 = u16;
pub type UInt32 = u32;

pub struct CPLi_DESC {
    pub cpli: UInt32,
    pub flags: UInt32,
    pub min_power_cap: UInt32,
    pub max_power_cap: UInt32,
    pub power_cap_step: UInt32,
    pub min_cai: UInt32,
    pub max_cai: UInt32,
    pub cai_step: UInt32,
}

pub struct S {
    pub domain: UInt32,
    pub index: UInt32,
    pub cpl_count: UInt16,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const NOT_FOUND: Int32 = -4;
pub const OUT_OF_RANGE: Int32 = -6;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn domain_id(s: S) -> UInt32;

pub open spec fn desc_index(s: S) -> UInt32;

pub open spec fn total_cpl_count(s: S) -> UInt16;

pub open spec fn PowercapDomainExists(domain_id: UInt32) -> bool;

pub open spec fn IsValidCpliDescIndex(domain_id: UInt32, desc_index: UInt32) -> bool;

pub open spec fn IsRequestSupported() -> bool;

pub open spec fn DomainSupportsCpc(domain_id: UInt32) -> bool;

pub open spec fn CpliDescriptor(domain_id: UInt32, desc_index: UInt32) -> CPLi_DESC;

pub open spec fn IsAscending(f: spec_fn(UInt16) -> UInt32) -> bool;

} // verus!
