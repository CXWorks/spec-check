use vstd::prelude::*;

verus! {

pub type Address = u64;
pub type UInt64 = u64;
pub type Int64 = i64;

pub enum RmiStatusCode {
    NotSupported,
    InvalidParameters,
    InvalidData,
    OutOfResource,
    Denied,
}

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Result<(), RmiStatusCode> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), RmiStatusCode> = Err(RmiStatusCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<(), RmiStatusCode> = Err(RmiStatusCode::InvalidParameters);
pub spec const INVALID_DATA: Result<(), RmiStatusCode> = Err(RmiStatusCode::InvalidData);
pub spec const OUT_OF_RESOURCE: Result<(), RmiStatusCode> = Err(RmiStatusCode::OutOfResource);
pub spec const DENIED: Result<(), RmiStatusCode> = Err(RmiStatusCode::Denied);

pub open spec fn ResultEqual(a: Result<(), RmiStatusCode>, b: Result<(), RmiStatusCode>) -> bool;
pub open spec fn DrtmIsSupported(s: S) -> bool;
pub open spec fn IsValidTcbHashTableAddress(s: S, addr: Address) -> bool;
pub open spec fn IsValidTcbHashTableHeader(s: S, addr: Address) -> bool;
pub open spec fn TcbHashTableEntryCount(s: S, addr: Address) -> UInt64;
pub open spec fn IsValidTcbHashTableEntry(s: S, addr: Address, i: UInt64) -> bool;
pub open spec fn FirstInvalidTcbHashTableEntry(s: S, addr: Address) -> Int64;
pub open spec fn ExceedsMaxTcbHashes(s: S, addr: Address) -> bool;
pub open spec fn TcbHashesLocked(s: S) -> bool;
pub open spec fn TcbHashesRecorded(s: S, addr: Address) -> bool;
pub open spec fn SourceOfEntryIgnored(s: S, addr: Address) -> bool;
pub open spec fn NumValidTcbHashEntries(s: S) -> Int64;

} // verus!
