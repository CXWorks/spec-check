use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u8,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -4;

pub open spec fn PerfDomainIsValid(s: S, domain_id: u32) -> bool;
pub open spec fn PerfDomainAgentCanSetLimits(s: S, domain_id: u32) -> bool;
pub open spec fn PerfDomainAgentCanSetLevel(s: S, domain_id: u32) -> bool;
pub open spec fn PerfDomainLimitsNotifySupported(s: S, domain_id: u32) -> bool;
pub open spec fn PerfDomainLevelNotifySupported(s: S, domain_id: u32) -> bool;
pub open spec fn PerfDomainHasFastChannel(s: S, domain_id: u32) -> bool;
pub open spec fn PerfDomainHasExtendedName(s: S, domain_id: u32) -> bool;
pub open spec fn PerfDomainUsesLevelIndexingMode(s: S, domain_id: u32) -> bool;
pub open spec fn PerfDomainAsyncQosSupported(s: S, domain_id: u32) -> bool;
pub open spec fn PerfDomainIsQosOnly(s: S, domain_id: u32) -> bool;
pub open spec fn PerfDomainSustainedReductionSupported(s: S, domain_id: u32) -> bool;
pub open spec fn PerfDomainRateLimit(s: S, domain_id: u32) -> u32;
pub open spec fn PerfDomainSustainedFreq(s: S, domain_id: u32) -> u32;
pub open spec fn PerfDomainSustainedPerfLevel(s: S, domain_id: u32) -> u32;
pub open spec fn IsNullTerminatedAscii(name: Seq<u8>) -> bool;
pub open spec fn PerfDomainName(s: S, domain_id: u32) -> Seq<u8>;
pub open spec fn IsNullTerminatedPrefixOf(name: Seq<u8>, full: Seq<u8>) -> bool;
pub open spec fn PerfDomainGuaranteedPerfLevel(s: S, domain_id: u32) -> u32;
pub open spec fn PerfDomainQosCapabilityTypes(s: S, domain_id: u32) -> u32;
pub open spec fn PerfDomainHasQosParent(s: S, domain_id: u32) -> bool;
pub open spec fn PerfDomainQosParentId(s: S, domain_id: u32) -> u32;

} // verus!
