use vstd::prelude::*;

verus! {

pub type uint32 = u32;
pub type int32 = i32;
pub type NegotiatedProtocolVersion = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;

pub open spec fn IsProtocolVersionSupported(s: S, version: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

#[allow(non_snake_case)]
pub open spec fn NegotiatedProtocolVersion(s: S) -> uint32;

} // verus!
