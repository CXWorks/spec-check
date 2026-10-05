use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const clock_id: uint32 = 0u32;

pub spec const SUCCESS: int32 = 0i32;
pub spec const NOT_SUPPORTED: int32 = -1i32;
pub spec const NOT_FOUND: int32 = -4i32;
pub spec const DENIED: int32 = -3i32;

pub spec const CLOCK_GET_PERMISSIONS: uint32 = 15u32;
pub spec const CLOCK_CONFIG_SET: uint32 = 5u32;
pub spec const CLOCK_PARENT_SET: uint32 = 13u32;
pub spec const CLOCK_RATE_SET: uint32 = 6u32;

pub open spec fn IsValidClockId(id: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn IsRequestSupported(msg: uint32) -> bool;

pub open spec fn AgentCanChangeClockState(id: uint32) -> bool;

pub open spec fn AgentCanChangeClockParent(id: uint32) -> bool;

pub open spec fn AgentCanChangeClockRate(id: uint32) -> bool;

} // verus!
