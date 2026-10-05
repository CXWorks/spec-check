use vstd::prelude::*;
verus! {

pub type uint32 = u32;
pub type int32 = i32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_SUPPORTED: int32 = -1;
pub spec const NOT_FOUND: int32 = -4;
pub spec const DENIED: int32 = -3;

pub spec const CLOCK_GET_PERMISSIONS: uint32 = 17;
pub spec const CLOCK_CONFIG_SET: uint32 = 5;
pub spec const CLOCK_PARENT_SET: uint32 = 13;
pub spec const CLOCK_RATE_SET: uint32 = 7;

pub spec const result: int32 = 100;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn IsValidClockId(s: S, clock_id: uint32) -> bool;

pub open spec fn IsRequestSupported(s: S, msg_id: uint32) -> bool;

pub open spec fn AgentCanChangeClockState(s: S, clock_id: uint32) -> bool;

pub open spec fn AgentCanChangeClockParent(s: S, clock_id: uint32) -> bool;

pub open spec fn AgentCanChangeClockRate(s: S, clock_id: uint32) -> bool;

} // verus!
