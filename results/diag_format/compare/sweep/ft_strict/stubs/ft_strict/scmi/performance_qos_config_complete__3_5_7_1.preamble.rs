use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type PerfDomainId = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> int;

pub open spec fn Bit(x: UInt32, pos: int) -> int;

pub open spec fn QosValue(s: S, d: PerfDomainId, capability: UInt32) -> UInt32;

pub open spec fn PlatformDefaultQos(s: S, d: PerfDomainId, capability: UInt32) -> UInt32;

pub open spec fn IsSiblingDomain(s: S, d: PerfDomainId, other: PerfDomainId) -> bool;

} // verus!
