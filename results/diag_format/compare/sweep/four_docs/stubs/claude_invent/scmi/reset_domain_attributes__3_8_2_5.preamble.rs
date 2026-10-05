use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub const SUCCESS: Int32 = 0i32;
pub const NOT_FOUND: Int32 = -4i32;

pub struct S {
    pub dummy: nat,
}

pub open spec fn ResetDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResetDomainSupportsAsyncReset(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResetDomainSupportsResetNotifications(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResetDomainHasExtendedName(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResetDomainLatencySupported(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResetDomainMaxLatency(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn IsNullTerminatedAsciiString(name: Seq<u8>) -> bool;

pub open spec fn ResetDomainName(s: S, domain_id: UInt32) -> Seq<u8>;

} // verus!
