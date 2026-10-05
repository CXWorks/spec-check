use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type UInt32 = u32;

pub type AgentId = u64;

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;

pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub const RSI_ERROR_NOT_FOUND: RsiCommandReturnCode = 2;

pub const RSI_ERROR_DENIED: RsiCommandReturnCode = 3;

pub const RSI_ERROR_IN_USE: RsiCommandReturnCode = 4;

pub struct S {
    pub cmd_input_0: u32,
    pub cmd_input_1: u64,
}

pub struct PinOrGroup {
    pub owner: AgentId,
}

pub uninterp spec fn Bits64(x: u64, hi: u64, lo: u64) -> u64;

pub uninterp spec fn CallingAgent() -> AgentId;

pub uninterp spec fn IsValidPinOrGroup(id: u32, kind: u64) -> bool;

pub uninterp spec fn AgentMayRequestPinOrGroup(agent: AgentId, id: u32, kind: u64) -> bool;

pub uninterp spec fn IsUnderExclusiveControlOfOtherAgent(id: u32, kind: u64, agent: AgentId) -> bool;

pub uninterp spec fn PinOrGroupAt(id: u32, kind: u64) -> PinOrGroup;

pub uninterp spec fn PinOrGroupAvailableTo(agent: AgentId, id: u32, kind: u64) -> bool;

} // verus!
