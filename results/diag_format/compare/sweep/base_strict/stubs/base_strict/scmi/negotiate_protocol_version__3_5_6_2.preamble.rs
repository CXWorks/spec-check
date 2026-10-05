use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub negotiated_protocol_version: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;

#[allow(non_upper_case_globals)]
pub const version: UInt32 = 0x20000;

pub open spec fn IsProtocolVersionSupported(v: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, expected: Int32) -> bool;

pub open spec fn NegotiatedProtocolVersion(s: S) -> UInt32;

pub open spec fn CommandsResponsesNotificationsComplyWithVersion(s: S, v: UInt32) -> bool;

} // verus!
