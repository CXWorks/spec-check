use vstd::prelude::*;
verus! {

pub type RmiStatusCode = u64;

pub const RMI_SUCCESS: RmiStatusCode = 0;
pub const RMI_ERROR_NOT_FOUND: RmiStatusCode = 1;

pub struct S {
    pub domain_id: u64,
    pub cap_config: u64,
    pub power_monitor: u64,
    pub power_unit: u64,
    pub num_limits: u64,
    pub reserved: u64,
    pub min_mai: u64,
    pub max_mai: u64,
    pub mai_step: u64,
    pub min_power_cap: u64,
    pub max_power_cap: u64,
    pub power_cap_step: u64,
    pub min_cai: u64,
    pub max_cai: u64,
    pub cai_step: u64,
}

pub open spec fn PowercapDomainExists(s: S, domain_id: u32) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn ValidPowercapDomainAttributes(old_s: S, new_s: S) -> bool;

pub open spec fn MaiIsConfigurable(s: S, domain_id: u32) -> bool;

pub open spec fn PowerCapIsConfigurable(s: S, domain_id: u32) -> bool;

pub open spec fn CaiIsConfigurable(s: S, domain_id: u32) -> bool;

} // verus!
