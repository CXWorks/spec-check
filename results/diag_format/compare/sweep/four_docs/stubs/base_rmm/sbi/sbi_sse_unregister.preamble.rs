use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type RsiCommandReturnCode = u64;

pub type SseEventStateValue = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;

pub const UNUSED: SseEventStateValue = 0;
pub const REGISTERED: SseEventStateValue = 1;
pub const ENABLED: SseEventStateValue = 2;
pub const RUNNING: SseEventStateValue = 3;

pub struct CmdInput {
    pub event_id: UInt64,
}

pub struct S {
    pub cmd_input: CmdInput,
}

pub open spec fn IsLocalEvent(event_id: UInt64) -> bool;

pub open spec fn IsGlobalEvent(event_id: UInt64) -> bool;

pub open spec fn SseEventState(event_id: UInt64) -> SseEventStateValue;

pub open spec fn IsHandlerRegistered(event_id: UInt64) -> bool;

} // verus!
