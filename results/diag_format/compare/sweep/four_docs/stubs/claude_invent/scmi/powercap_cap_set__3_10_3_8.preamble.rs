use vstd::prelude::*;

verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const INVALID_PARAMETERS: i32 = -2;
pub const DENIED: i32 = -3;
pub const NOT_FOUND: i32 = -4;

pub open spec fn PowercapDomainValid(s: S, domain_id: u32) -> bool;

pub open spec fn PowercapCpliValid(s: S, domain_id: u32, cpli: u32) -> bool;

pub open spec fn PowercapCapSetRequestSupported(s: S, domain_id: u32, flags: u32) -> bool;

pub open spec fn PowercapCapValueSupported(s: S, domain_id: u32, cpli: u32, power_cap: u32) -> bool;

pub open spec fn PowercapAgentAllowed(s: S, domain_id: u32) -> bool;

pub open spec fn PowercapAgentCapRequest(s: S, domain_id: u32, cpli: u32) -> u32;

pub open spec fn PowercapCapSetEnqueued(s: S, domain_id: u32, cpli: u32, power_cap: u32) -> bool;

pub open spec fn PowercapCapSetCompletePending(s: S, domain_id: u32, cpli: u32) -> bool;

} // verus!
