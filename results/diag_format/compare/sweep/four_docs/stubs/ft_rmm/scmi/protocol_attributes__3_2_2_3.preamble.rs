use vstd::prelude::*;
verus! {

pub type uint32 = u32;

pub struct S {
    pub agent_discovery_supported: bool,
    pub num_agents: u32,
    pub num_protocols: u32,
}

pub open spec fn NumAgentsInSystem() -> u32;

pub open spec fn NumImplementedProtocolsExcludingBase() -> u32;

pub open spec fn PlatformSupportsAgentDiscovery(s: S) -> bool;

} // verus!
