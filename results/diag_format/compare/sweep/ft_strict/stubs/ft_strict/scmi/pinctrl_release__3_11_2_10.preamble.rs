use vstd::prelude::*;
verus! {

pub type uint32 = u32;
pub type int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: int32 = 0;
pub spec const INVALID_PARAMETERS: int32 = (-2int) as int32;
pub spec const NOT_FOUND: int32 = (-4int) as int32;
#[allow(non_upper_case_globals)]
pub spec const result: int32 = (-100int) as int32;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;
pub open spec fn IsValidPinOrGroup(s: S, identifier: uint32, sel: uint32) -> bool;
pub open spec fn CallingAgent(s: S) -> AgentId;
pub open spec fn HasExclusiveControl(s: S, agent: AgentId, identifier: uint32, sel: uint32) -> bool;

} // verus!
