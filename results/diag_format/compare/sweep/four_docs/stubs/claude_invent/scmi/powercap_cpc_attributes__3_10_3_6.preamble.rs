use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const OUT_OF_RANGE: i32 = -6;
pub const NOT_FOUND: i32 = -4;

pub struct CpliDesc {
    pub cpli: u32,
    pub flags: u32,
    pub min_power_cap: u32,
    pub max_power_cap: u32,
    pub power_cap_step: u32,
    pub min_cai: u32,
    pub max_cai: u32,
    pub cai_step: u32,
    pub name: Seq<u8>,
}

pub struct S {
    pub domains: Seq<u32>,
}

pub open spec fn PowercapDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn PowercapDomainSupportsCpc(s: S, domain_id: UInt32) -> bool;

pub open spec fn PowercapCpcNumCpli(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PowercapCpcCpliDescAt(s: S, domain_id: UInt32, index: int) -> CpliDesc;

} // verus!
