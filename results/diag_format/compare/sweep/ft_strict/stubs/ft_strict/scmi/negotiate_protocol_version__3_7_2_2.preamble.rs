use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type RmiStatusCode = u64;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(self) -> bool {
        match self {
            Result::Ok(_) => true,
            Result::Err(_) => false,
        }
    }

    pub open spec fn is_Err(self) -> bool {
        match self {
            Result::Ok(_) => false,
            Result::Err(_) => true,
        }
    }
}

pub struct S {
    pub negotiated_protocol_version: UInt32,
}

pub const SUCCESS: RmiStatusCode = 0;

pub const NOT_SUPPORTED: RmiStatusCode = 1;

pub open spec fn IsSupportedProtocolVersion(s: S, version: UInt32) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn NegotiatedProtocolVersion(s: S) -> UInt32;

} // verus!
