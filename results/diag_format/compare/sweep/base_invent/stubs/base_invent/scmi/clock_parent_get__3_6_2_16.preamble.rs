use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const NOT_FOUND: int32 = -4;
pub const DENIED: int32 = -3;

#[allow(non_upper_case_globals)]
pub const clock_id: uint32 = 0;

pub open spec fn ClockExists(s: S, id: uint32) -> bool;
pub open spec fn ClockSupported(s: S, id: uint32) -> bool;
pub open spec fn AgentAllowedToGetParent(s: S, id: uint32) -> bool;
pub open spec fn ClockParent(s: S, id: uint32) -> uint32;

} // verus!
