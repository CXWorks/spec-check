use vstd::prelude::*;

verus! {

pub struct TcbHashEntry {
    pub hash_id: u32,
    pub digest: Seq<u8>,
}

pub struct S {
    pub drtm_supported: bool,
    pub tcb_hashes: Seq<TcbHashEntry>,
    pub tcb_hashes_locked: bool,
    pub max_tcb_hashes: u64,
    pub memory: Map<u64, u8>,
}

pub const SUCCESS: i64 = 0;
pub const NOT_SUPPORTED: i64 = -1;
pub const INVALID_PARAMETERS: i64 = -2;
pub const DENIED: i64 = -3;
pub const INVALID_DATA: i64 = -4;
pub const OUT_OF_RESOURCE: i64 = -5;

pub open spec fn DrtmSupported(s: S) -> bool;

pub open spec fn TcbHashTableAddrValid(s: S, addr: u64) -> bool;

pub open spec fn TcbHashTableHeaderValid(s: S, addr: u64) -> bool;

pub open spec fn TcbHashTableHasInvalidEntry(s: S, addr: u64) -> bool;

pub open spec fn TcbHashTableFirstInvalidEntryIndex(s: S, addr: u64) -> i64;

pub open spec fn TcbHashCount(s: S) -> u64;

pub open spec fn TcbHashTableNumEntries(s: S, addr: u64) -> u64;

pub open spec fn DrtmMaxTcbHashes(s: S) -> u64;

pub open spec fn TcbHashesLocked(s: S) -> bool;

pub open spec fn TcbHashes(s: S) -> Seq<TcbHashEntry>;

pub open spec fn TcbHashTableEntries(s: S, addr: u64) -> Seq<TcbHashEntry>;

pub open spec fn TcbHashesAppendIgnoringSource(existing: Seq<TcbHashEntry>, entries: Seq<TcbHashEntry>) -> Seq<TcbHashEntry>;

} // verus!
