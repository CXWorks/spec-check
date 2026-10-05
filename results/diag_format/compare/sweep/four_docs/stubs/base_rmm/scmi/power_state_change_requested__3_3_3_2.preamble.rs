use vstd::prelude::*;
verus! {

pub type RsiCommandReturnCode = u64;
pub type AgentId = u32;
pub type DomainId = u32;
pub type PowerState = u32;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub const recipient_agent: AgentId = 1;
pub const agent_id: AgentId = 2;
pub const domain_id: DomainId = 3;
pub const power_state: PowerState = 4;

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsRegisteredForPowerStateChangeRequested(s: S, agent: AgentId) -> bool;

pub open spec fn PowerStateChangeRequestReceived(s: S, agent: AgentId, domain: DomainId, state: PowerState) -> bool;

pub open spec fn NotificationSentTo(s: S, recipient: AgentId, agent: AgentId, domain: DomainId, state: PowerState) -> bool;

} // verus!
