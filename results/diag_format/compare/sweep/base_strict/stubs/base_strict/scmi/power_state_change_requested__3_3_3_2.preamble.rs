use vstd::prelude::*;

verus! {

pub type RsiCommandReturnCode = u64;

pub type AgentId = u32;

pub type DomainId = u32;

pub type PowerState = u32;

pub spec const RSI_SUCCESS: RsiCommandReturnCode = 0;

pub spec const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub spec const recipient_agent: AgentId = 1;

pub spec const agent_id: AgentId = 2;

pub spec const domain_id: DomainId = 3;

pub spec const power_state: PowerState = 4;

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsRegisteredForPowerStateChangeRequested(s: S, agent: AgentId) -> bool;

pub open spec fn PlatformReceivedPowerStateChangeRequest(s: S, agent: AgentId, domain: DomainId, state: PowerState) -> bool;

} // verus!
