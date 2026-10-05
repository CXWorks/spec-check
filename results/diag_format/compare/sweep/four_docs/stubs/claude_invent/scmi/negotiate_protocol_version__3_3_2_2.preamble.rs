use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: i32 = 0i32;
pub spec const NOT_SUPPORTED: i32 = -1i32;

pub uninterp spec fn IsProtocolVersionSupported(s: S, protocol_id: u32, version: UInt32) -> bool;

pub uninterp spec fn NegotiatedProtocolVersion(s: S, protocol_id: u32) -> UInt32;

} // verus!
