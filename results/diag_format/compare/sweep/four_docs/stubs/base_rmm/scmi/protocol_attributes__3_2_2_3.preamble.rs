use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub struct uint32 {
    pub value: u32,
}

impl uint32 {
    pub open spec fn spec_index(self, r: core::ops::Range<int>) -> int;
}

pub struct S {
    pub num_agents: int,
    pub num_protocols: int,
}

pub open spec fn NumAgentsInSystem() -> int;

pub open spec fn NumImplementedProtocolsExcludingBase() -> int;

pub open spec fn PlatformSupportsAgentDiscovery() -> bool;

} // verus!
