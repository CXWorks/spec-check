use vstd::prelude::*;
verus! {

pub type UInt32 = Seq<u32>;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub agents: Seq<AgentId>,
}

pub const caller: AgentId = 0;

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;

pub open spec fn NotificationsSupportedForAgent(s: S, agent: AgentId) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidNotifyEnable(s: S, notify_enable: UInt32) -> bool;

pub open spec fn SystemPowerStateNotifyEnabled(s: S, agent: AgentId) -> bool;

} // verus!
