use vstd::prelude::*;

verus! {

pub type RmiStatusCode = u64;

pub type ProtocolVersion = u64;

pub struct S {
    pub negotiated_version: ProtocolVersion,
}

pub const SUCCESS: RmiStatusCode = 0;

pub const NOT_SUPPORTED: RmiStatusCode = 1;

pub const version: ProtocolVersion = 2;

pub open spec fn IsProtocolVersionSupported(v: ProtocolVersion) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn NegotiatedProtocolVersion() -> ProtocolVersion;

pub open spec fn SubsequentMessagesComplyWithVersion(v: ProtocolVersion) -> bool;

} // verus!
