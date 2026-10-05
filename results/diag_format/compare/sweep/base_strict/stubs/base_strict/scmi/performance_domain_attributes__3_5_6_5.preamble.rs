use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub struct S {
    pub current_domain_id: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn domain_id(s: S) -> u32;

pub open spec fn IsValidPerformanceDomain(d: u32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn CallerCanSetPerfLimits(d: u32) -> bool;

pub open spec fn CallerCanSetPerfLevel(d: u32) -> bool;

pub open spec fn SupportsPerfLimitsChangeNotify(d: u32) -> bool;

pub open spec fn SupportsPerfLevelChangeNotify(d: u32) -> bool;

pub open spec fn HasFastChannel(d: u32) -> bool;

pub open spec fn PerfDomainNameLength(d: u32) -> int;

pub open spec fn UsesLevelIndexingMode(d: u32) -> bool;

pub open spec fn SupportsAsyncQosConfig(d: u32) -> bool;

pub open spec fn IsQosOnlyDomain(d: u32) -> bool;

pub open spec fn SupportsSustainedPerfReduction(d: u32) -> bool;

pub open spec fn RateLimitMicroseconds(d: u32) -> UInt32;

pub open spec fn SustainedFreqKhz(d: u32) -> UInt32;

pub open spec fn PlatformSustainedPerfLevel(d: u32) -> UInt32;

pub open spec fn IsNullTerminatedAscii(name: [UInt8; 16], len: int) -> bool;

pub open spec fn PerfDomainName(d: u32) -> [UInt8; 16];

pub open spec fn LowerBytesNullTerminated(name: [UInt8; 16], n: int) -> [UInt8; 16];

pub open spec fn GuaranteedPerfLevel(d: u32) -> UInt32;

pub open spec fn SupportedOemQosCapabilityTypes(d: u32) -> UInt32;

pub open spec fn SupportedArchQosCapabilityTypes(d: u32) -> UInt32;

pub open spec fn HasQosParentDomain(d: u32) -> bool;

pub open spec fn QosParentDomainId(d: u32) -> UInt32;

} // verus!
