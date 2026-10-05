use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const NOT_SUPPORTED: i32 = -1;
pub const INVALID_PARAMETERS: i32 = -2;
pub const DENIED: i32 = -3;

pub open spec fn CallerIsPsciImplementation(s: S) -> bool;

pub open spec fn SystemImplementsOspmSystemView(s: S) -> bool;

pub open spec fn IsSystemPowerStateSupportedForCaller(s: S, system_state: UInt32) -> bool;

pub open spec fn OtherApplicationProcessorsRunningOrIdle(s: S) -> bool;

} // verus!
