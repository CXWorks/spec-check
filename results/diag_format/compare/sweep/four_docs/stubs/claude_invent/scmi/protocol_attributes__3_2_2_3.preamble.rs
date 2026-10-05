use vstd::prelude::*;

verus! {

pub struct S {
    pub agent_discovery_supported: bool,
    pub num_agents: int,
    pub num_protocols_excluding_base: int,
}

pub open spec fn PlatformSupportsAgentDiscovery(s: S) -> bool;

pub open spec fn NumAgentsInSystem(s: S) -> int;

pub open spec fn NumImplementedProtocolsExcludingBase(s: S) -> int;

} // verus!
