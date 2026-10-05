use vstd::prelude::*;

verus! {

pub type RsiCommandReturnCode = u64;
pub type AgentId = u64;
pub type NotificationId = u64;
pub type UInt64 = u64;

pub spec const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub spec const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub spec const RSI_ERROR_STATE: RsiCommandReturnCode = 2;
pub spec const RSI_INCOMPLETE: RsiCommandReturnCode = 3;

pub spec const SYSTEM_POWER_STATE_NOTIFIER: NotificationId = 100;

pub spec const agent: AgentId = 11;
pub spec const flags: UInt64 = 12;
pub spec const system_state: UInt64 = 13;
pub spec const timeout: UInt64 = 14;

pub struct S {
    pub registered_agents: Set<AgentId>,
    pub sent_notifications: Set<(AgentId, NotificationId)>,
}

pub uninterp spec fn IsRegisteredForSystemPowerStateNotify(s: S, a: AgentId) -> bool;

pub uninterp spec fn NotificationSentToAgent(s: S, a: AgentId, n: NotificationId) -> bool;

pub uninterp spec fn PlatformImposesShutdownTimeout() -> bool;

} // verus!
