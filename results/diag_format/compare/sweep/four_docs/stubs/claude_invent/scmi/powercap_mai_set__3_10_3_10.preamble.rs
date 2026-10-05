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

pub open spec fn PowercapDomainExists(s: S, domain_id: u32) -> bool;
pub open spec fn PowercapMaiConfigSupported(s: S, domain_id: u32) -> bool;
pub open spec fn PowercapMaiSupported(s: S, domain_id: u32, mai: u32) -> bool;
pub open spec fn PowercapAgentAllowedSetMai(s: S, domain_id: u32) -> bool;
pub open spec fn PowercapDomainMai(s: S, domain_id: u32) -> u32;
pub open spec fn PowercapStateUnchangedExceptMai(old_s: S, new_s: S, domain_id: u32) -> bool;

} // verus!
