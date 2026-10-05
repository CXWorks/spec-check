use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const DENIED: int32 = -3;
pub const NOT_FOUND: int32 = -4;

pub const clock_id: uint32 = 1;
pub const calling_agent: uint32 = 2;

pub uninterp spec fn ClockExists(s: S, cid: uint32) -> bool;
pub uninterp spec fn IsRequestSupported(s: S, cid: uint32) -> bool;
pub uninterp spec fn AgentMayGetClockParent(s: S, agent_id: uint32, cid: uint32) -> bool;
pub uninterp spec fn ResultEqual(result: int32, code: int32) -> bool;
pub uninterp spec fn ClockParent(s: S, cid: uint32) -> uint32;

} // verus!
