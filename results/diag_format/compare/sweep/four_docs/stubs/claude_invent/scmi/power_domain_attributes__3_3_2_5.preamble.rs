use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub power_domain_count: nat,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -4;

pub open spec fn PowerDomainExists(s: S, domain_id: UInt32) -> bool;
pub open spec fn PowerDomainStateChangeNotifySupported(s: S, domain_id: UInt32) -> bool;
pub open spec fn PowerDomainAsyncSetSupported(s: S, domain_id: UInt32) -> bool;
pub open spec fn PowerDomainSyncSetSupported(s: S, domain_id: UInt32) -> bool;
pub open spec fn PowerDomainStateChangeRequestedNotifySupported(s: S, domain_id: UInt32) -> bool;
pub open spec fn PowerDomainHasExtendedName(s: S, domain_id: UInt32) -> bool;
pub open spec fn PowerDomainNameMatches(s: S, domain_id: UInt32, name: [u8; 16], extended: bool) -> bool;

} // verus!
