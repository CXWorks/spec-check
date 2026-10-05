use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_FOUND: int32 = (-4) as int32;

pub spec const message_id: uint32 = 3;

pub open spec fn IsMessageProvided(s: S, msg_id: uint32) -> bool;

pub open spec fn IsMessageImplementedAndAvailable(s: S, msg_id: uint32) -> bool;

pub open spec fn ResultEqual(result: int32, expected: int32) -> bool;

} // verus!
