use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

pub struct PinOrGroup {
    pub owner: AgentId,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;
pub const IN_USE: Int32 = -6;

#[allow(non_upper_case_globals)]
pub const result: Int32 = -100;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn IsValidPinOrGroup(s: S, identifier: UInt32, selector: int) -> bool;

pub open spec fn CallingAgent(s: S) -> AgentId;

pub open spec fn AgentMayRequestPinOrGroup(s: S, agent: AgentId, identifier: UInt32, selector: int) -> bool;

pub open spec fn IsUnderExclusiveControlOfOtherAgent(s: S, identifier: UInt32, selector: int, agent: AgentId) -> bool;

pub open spec fn PinOrGroupAt(s: S, identifier: UInt32, selector: int) -> PinOrGroup;

pub open spec fn PinOrGroupAvailableTo(s: S, agent: AgentId, identifier: UInt32, selector: int) -> bool;

} // verus!
