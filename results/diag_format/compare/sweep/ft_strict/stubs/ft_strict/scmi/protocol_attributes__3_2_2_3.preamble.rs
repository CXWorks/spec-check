use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub open spec fn Bits(value: UInt32, high: int, low: int) -> int;

pub open spec fn PlatformSupportsAgentDiscovery(s: S) -> bool;

pub open spec fn NumAgentsInSystem(s: S) -> int;

pub open spec fn NumImplementedProtocolsExcludingBase(s: S) -> int;

} // verus!
