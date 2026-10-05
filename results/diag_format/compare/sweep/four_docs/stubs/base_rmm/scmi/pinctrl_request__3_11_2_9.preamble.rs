use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub cmd_input_param_0: UInt32,
    pub cmd_input_param_1: UInt32,
    pub cmd_input_caller: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;
pub const IN_USE: Int32 = -9;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidPinOrGroup(id: UInt32, selector: UInt32) -> bool;

pub open spec fn AgentMayRequestPinOrGroup(agent: UInt32, id: UInt32, selector: UInt32) -> bool;

pub open spec fn IsUnderExclusiveControl(id: UInt32, selector: UInt32) -> bool;

pub open spec fn ExclusiveOwner(id: UInt32, selector: UInt32) -> UInt32;

pub open spec fn flags_rsvd(old_s: S) -> bool;

pub open spec fn flags_selector(old_s: S) -> bool;

pub open spec fn id_valid(old_s: S) -> bool;

pub open spec fn agent_permitted(old_s: S) -> bool;

pub open spec fn in_use(old_s: S) -> bool;

pub open spec fn owner(old_s: S, new_s: S) -> bool;

} // verus!
