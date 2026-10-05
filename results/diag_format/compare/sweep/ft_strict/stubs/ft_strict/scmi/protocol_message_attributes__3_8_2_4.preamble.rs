use vstd::prelude::*;

verus! {

pub type uint32 = u32;

pub type int32 = i32;

pub struct S {
    pub dummy: int,
}

pub enum RmiStatusCode {
    Success,
    NotFound,
    InvalidParameters,
    Denied,
}

pub const SUCCESS: int32 = 0;

pub const NOT_FOUND: int32 = -4;

pub open spec fn IsMessageProvided(s: S, protocol_id: u32, message_id: uint32) -> bool;

pub open spec fn IsMessageImplementedAndAvailable(s: S, protocol_id: u32, message_id: uint32) -> bool;

pub open spec fn ResultEqual(status: int32, expected: int32) -> bool;

} // verus!
