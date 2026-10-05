use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub negotiated_version: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -2;

pub spec const result: bool = arbitrary();

pub open spec fn IsProtocolVersionSupported(s: S, version: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

pub open spec fn NegotiatedProtocolVersion(s: S) -> UInt32;

pub open spec fn SubsequentMessagesComplyWithVersion(s: S, version: UInt32) -> bool;

} // verus!
