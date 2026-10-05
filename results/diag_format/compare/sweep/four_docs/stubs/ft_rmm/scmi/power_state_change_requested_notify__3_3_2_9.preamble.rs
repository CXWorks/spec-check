use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt64 = u64;

pub enum RmiStatusCode {
    RmiSuccess,
    RmiErrorInput,
    RmiErrorNotFound,
    RmiErrorState,
}

pub struct S {
    pub power_domain_count: UInt32,
    pub notify_enabled: Map<(UInt64, UInt32), UInt32>,
    pub valid_power_domains: Set<UInt32>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;

pub const caller: UInt64 = 0;

pub open spec fn IsValidPowerDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn PowerStateChangeRequestedNotifyEnabled(s: S, agent_id: UInt64, domain_id: UInt32) -> UInt32;

} // verus!
