use vstd::prelude::*;
verus! {

pub type AgentId = u32;

pub struct S {
    pub caller: AgentId,
    pub dummy: u64,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const INVALID_PARAMETERS: i32 = -2;
pub const DENIED: i32 = -3;
pub const NOT_FOUND: i32 = -4;
pub const IN_USE: i32 = -9;

pub open spec fn CallerAgent(s: S) -> AgentId;

pub open spec fn PinctrlIdentifierValid(s: S, identifier: u32, sel: u32) -> bool;

pub open spec fn PinctrlAgentAllowed(s: S, agent: AgentId, identifier: u32, sel: u32) -> bool;

pub open spec fn PinctrlIsExclusivelyControlledByOther(s: S, agent: AgentId, identifier: u32, sel: u32) -> bool;

pub open spec fn PinctrlIsExclusivelyControlledBy(s: S, agent: AgentId, identifier: u32, sel: u32) -> bool;

} // verus!
