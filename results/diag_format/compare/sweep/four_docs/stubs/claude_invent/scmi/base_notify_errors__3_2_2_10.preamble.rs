use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub error_notify: Map<UInt32, bool>,
}

pub const SUCCESS: i32 = 0i32;
pub const INVALID_PARAMETERS: i32 = -1i32;

pub open spec fn ErrorNotifyEnabled(s: S, agent_id: UInt32) -> bool;

} // verus!
