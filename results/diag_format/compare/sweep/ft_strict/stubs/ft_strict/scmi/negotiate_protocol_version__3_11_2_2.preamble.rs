use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type Int32 = i32;

pub type RmiStatusCode = u32;

pub struct S {
    pub negotiated_protocol_version: UInt32,
    pub supported_versions: Set<UInt32>,
}

pub spec const NOT_SUPPORTED: RmiStatusCode = 1;

pub spec const SUCCESS: Int32 = 0;

pub open spec fn IsProtocolVersionSupported(s: S, version: UInt32) -> bool;

pub open spec fn ResultEqual<A, B>(a: A, b: B) -> bool;

pub open spec fn NegotiatedProtocolVersion(s: S) -> UInt32;

pub open spec fn SubsequentMessagesComplyWithVersion(s: S, version: UInt32) -> bool;

} // verus!
