use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub type UInt64 = u64;

pub type TcbHashTable = u64;

pub type TcbDigest = Seq<u8>;

pub struct S {
    pub recorded_tcb_hashes: Seq<TcbDigest>,
    pub tcb_hashes_locked: bool,
    pub memory: Map<u64, u8>,
}

pub spec const SUCCESS: Int64 = 0;
pub spec const NOT_SUPPORTED: Int64 = (-1int) as i64;
pub spec const INVALID_PARAMETERS: Int64 = (-2int) as i64;
pub spec const DENIED: Int64 = (-3int) as i64;
pub spec const INVALID_DATA: Int64 = (-4int) as i64;
pub spec const OUT_OF_RESOURCE: Int64 = (-5int) as i64;

pub spec const tcb_hash_table: TcbHashTable = 0x1000;

pub open spec fn DrtmIsSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn IsValidTcbHashTableAddress(s: S, table: TcbHashTable) -> bool;

pub open spec fn IsValidTcbHashTableHeader(s: S, table: TcbHashTable) -> bool;

pub open spec fn AllTcbHashTableEntriesValid(s: S, table: TcbHashTable) -> bool;

pub open spec fn RecordedTcbHashCount(s: S) -> int;

pub open spec fn TcbHashTableEntryCount(table: TcbHashTable) -> int;

pub open spec fn MaxTcbHashes() -> int;

pub open spec fn TcbHashesLocked(s: S) -> bool;

pub open spec fn RecordedTcbHashes(s: S) -> Seq<TcbDigest>;

pub open spec fn Concat(a: Seq<TcbDigest>, b: Seq<TcbDigest>) -> Seq<TcbDigest>;

pub open spec fn TcbHashTableDigests(table: TcbHashTable) -> Seq<TcbDigest>;

pub open spec fn ValidTcbHashTableEntryCount(table: TcbHashTable) -> Int64;

} // verus!
