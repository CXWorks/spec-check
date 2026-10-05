use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = 1;

pub const version: UInt32 = 0;
pub const NegotiatedProtocolVersion: UInt32 = 0;

pub uninterp spec fn PlatformSupportsProtocolVersion(v: UInt32) -> bool;
pub uninterp spec fn ResultEqual(a: Int32, b: Int32) -> bool;

} // verus!
