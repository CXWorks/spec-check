use vstd::prelude::*;

verus! {

pub type Address = u64;

pub type Int64 = i64;

pub type Digest = Seq<u8>;

pub enum DrtmStatusCode {
    NotSupported,
    InvalidParameters,
    InvalidData,
    OutOfResource,
    Denied,
}

pub struct S {
    pub recorded_tcb_hashes: Seq<Digest>,
    pub tcb_hashes_locked: bool,
    pub max_tcb_hashes: int,
}

pub open spec const SUCCESS: Result<(), DrtmStatusCode> = Ok(());

pub open spec const NOT_SUPPORTED: Result<(), DrtmStatusCode> = Err(DrtmStatusCode::NotSupported);

pub open spec const INVALID_PARAMETERS: Result<(), DrtmStatusCode> = Err(DrtmStatusCode::InvalidParameters);

pub open spec const INVALID_DATA: Result<(), DrtmStatusCode> = Err(DrtmStatusCode::InvalidData);

pub open spec const OUT_OF_RESOURCE: Result<(), DrtmStatusCode> = Err(DrtmStatusCode::OutOfResource);

pub open spec const DENIED: Result<(), DrtmStatusCode> = Err(DrtmStatusCode::Denied);

pub open spec fn ResultEqual(a: Result<(), DrtmStatusCode>, b: Result<(), DrtmStatusCode>) -> bool;

pub open spec fn DrtmIsSupported(s: S) -> bool;

pub open spec fn IsValidTcbHashTableAddress(s: S, addr: Address) -> bool;

pub open spec fn IsValidTcbHashTableHeader(s: S, addr: Address) -> bool;

pub open spec fn AllTcbHashTableEntriesValid(s: S, addr: Address) -> bool;

pub open spec fn RecordedTcbHashCount(s: S) -> int;

pub open spec fn TcbHashTableEntryCount(s: S, addr: Address) -> int;

pub open spec fn MaxTcbHashes(s: S) -> int;

pub open spec fn TcbHashesLocked(s: S) -> bool;

pub open spec fn RecordedTcbHashes(s: S) -> Seq<Digest>;

pub open spec fn Concat(a: Seq<Digest>, b: Seq<Digest>) -> Seq<Digest>;

pub open spec fn TcbHashTableDigests(s: S, addr: Address) -> Seq<Digest>;

pub open spec fn ValidTcbHashTableEntryCount(s: S, addr: Address) -> Int64;

} // verus!
