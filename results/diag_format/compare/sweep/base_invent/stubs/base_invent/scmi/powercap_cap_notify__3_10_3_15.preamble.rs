use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub protocol_message_attributes: uint32,
    pub domain_id: uint32,
    pub notify_enable: uint32,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -4;
pub const INVALID_PARAMETERS: int32 = -2;

pub open spec fn ResultEqual(result: int32, expected: int32) -> bool;

} // verus!
