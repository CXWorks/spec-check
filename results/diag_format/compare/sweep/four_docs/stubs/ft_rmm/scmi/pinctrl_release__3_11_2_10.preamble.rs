use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;
pub type Selector = u32;

pub struct Flags {
    pub selector: Selector,
    pub raw: u32,
}

pub struct S {
    pub num_pins: nat,
    pub num_groups: nat,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const NOT_FOUND: Int32 = -4;

#[allow(non_upper_case_globals)]
pub const caller: AgentId = 7;

#[allow(non_upper_case_globals)]
pub const result: Int32 = -100;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn IsValidPinOrGroup(s: S, identifier: UInt32, selector: Selector) -> bool;

pub open spec fn AreValidParameters(s: S, identifier: UInt32, flags: Flags) -> bool;

pub open spec fn HasExclusiveControl(s: S, agent: AgentId, identifier: UInt32, selector: Selector) -> bool;

} // verus!
