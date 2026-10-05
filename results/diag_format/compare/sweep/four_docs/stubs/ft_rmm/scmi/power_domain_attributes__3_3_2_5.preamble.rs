use vstd::prelude::*;
verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub enum RmiStatusCode {
    Success,
    NotFound,
    InvalidParameters,
    Denied,
}

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
    pub domains: Seq<int>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn ResultEqual<A, B>(a: A, b: B) -> bool;

pub open spec fn PowerDomainExists(s: S, domain_id: int) -> bool;

pub open spec fn PowerStateChangeNotifySupported(s: S, domain_id: int) -> bool;

pub open spec fn PowerStateAsyncSetSupported(s: S, domain_id: int) -> bool;

pub open spec fn PowerStateSyncSetSupported(s: S, domain_id: int) -> bool;

pub open spec fn PowerStateChangeRequestedNotifySupported(s: S, domain_id: int) -> bool;

pub open spec fn PowerDomainNameLength(s: S, domain_id: int) -> int;

pub open spec fn PowerDomainName(s: S, domain_id: int) -> [UInt8; 16];

pub open spec fn NullTerminated(x: [UInt8; 16]) -> [UInt8; 16];

} // verus!
