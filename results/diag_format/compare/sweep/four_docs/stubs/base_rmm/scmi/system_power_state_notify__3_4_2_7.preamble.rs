use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = Seq<u32>;
pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

impl S {
    pub open spec fn SystemPowerStateNotifyEnabled(self, agent: AgentId) -> bool;
}

pub spec const caller: AgentId = 0;

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;

pub open spec fn NotificationsSupportedForAgent(s: S, agent: AgentId) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn IsValidNotifyEnable(notify_enable: UInt32) -> bool;

pub open spec fn SystemPowerStateNotifyEnabled(agent: AgentId) -> bool;

} // verus!
