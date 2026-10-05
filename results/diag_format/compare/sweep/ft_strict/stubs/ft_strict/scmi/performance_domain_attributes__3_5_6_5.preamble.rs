use vstd::prelude::*;
verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub enum RmiStatusCode {
    Success,
    NotFound,
    InvalidParameters,
    Denied,
}

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;

pub open spec fn ResultEqual<A, B>(a: A, b: B) -> bool;

pub open spec fn IsValidPerformanceDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn CallerCanSetPerfLimits(s: S, domain_id: UInt32) -> bool;

pub open spec fn CallerCanSetPerfLevel(s: S, domain_id: UInt32) -> bool;

pub open spec fn SupportsPerfLimitsChangeNotify(s: S, domain_id: UInt32) -> bool;

pub open spec fn SupportsPerfLevelChangeNotify(s: S, domain_id: UInt32) -> bool;

pub open spec fn HasFastChannel(s: S, domain_id: UInt32) -> bool;

pub open spec fn PerfDomainNameLength(s: S, domain_id: UInt32) -> int;

pub open spec fn UsesLevelIndexingMode(s: S, domain_id: UInt32) -> bool;

pub open spec fn SupportsAsyncQosConfig(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsQosOnlyDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn SupportsSustainedPerfReduction(s: S, domain_id: UInt32) -> bool;

pub open spec fn RateLimitMicroseconds(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn SustainedFreqKhz(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PlatformSustainedPerfLevel(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn IsNullTerminatedAscii(s: S, name: [UInt8; 16], len: int) -> bool;

pub open spec fn PerfDomainName(s: S, domain_id: UInt32) -> [UInt8; 16];

pub open spec fn LowerBytesNullTerminated(s: S, name: [UInt8; 16], n: int) -> [UInt8; 16];

pub open spec fn GuaranteedPerfLevel(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn SupportedOemQosCapabilityTypes(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn SupportedArchQosCapabilityTypes(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn HasQosParentDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn QosParentDomainId(s: S, domain_id: UInt32) -> UInt32;

} // verus!
