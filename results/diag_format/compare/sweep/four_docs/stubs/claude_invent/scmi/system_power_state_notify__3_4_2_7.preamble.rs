use vstd::prelude::*;

verus! {

pub const SUCCESS: i32 = 0i32;
pub const NOT_SUPPORTED: i32 = -1i32;
pub const INVALID_PARAMETERS: i32 = -2i32;

pub struct S {
    pub notify_enabled: Map<u32, bool>,
    pub notify_supported: Map<u32, bool>,
}

pub open spec fn SystemPowerStateNotifySupported(s: S, agent_id: u32) -> bool;

pub open spec fn SystemPowerStateNotifyEnabled(s: S, agent_id: u32) -> bool;

pub open spec fn SystemPowerStateNotifyOthersUnchanged(old_s: S, new_s: S, agent_id: u32) -> bool;

} // verus!
