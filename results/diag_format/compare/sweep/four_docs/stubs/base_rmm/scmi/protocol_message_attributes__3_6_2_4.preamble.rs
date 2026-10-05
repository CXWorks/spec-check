use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: u64,
}

pub struct Header {
    pub message_id: UInt32,
}

pub const header: Header = Header { message_id: 0 };

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn IsMessageImplemented(s: S, message_id: UInt32) -> bool;

pub open spec fn IsMessageAvailableToAgent(s: S, message_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

} // verus!
