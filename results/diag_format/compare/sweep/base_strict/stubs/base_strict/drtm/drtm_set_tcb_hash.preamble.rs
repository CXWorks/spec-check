use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type UInt64 = u64;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int64 = 0;
pub spec const NOT_SUPPORTED: Int64 = (-1) as i64;
pub spec const INVALID_PARAMETERS: Int64 = (-2) as i64;
pub spec const DENIED: Int64 = (-3) as i64;
pub spec const OUT_OF_RESOURCE: Int64 = (-4) as i64;
pub spec const INVALID_DATA: Int64 = (-5) as i64;

pub spec const tcb_hash_table: UInt64 = 0x1000;

pub uninterp spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub uninterp spec fn DrtmIsSupported() -> bool;

pub uninterp spec fn IsValidTcbHashTableAddress(s: S, table: UInt64) -> bool;

pub uninterp spec fn IsValidTcbHashTableHeader(s: S, table: UInt64) -> bool;

pub uninterp spec fn TcbHashTableEntryCount(table: UInt64) -> UInt64;

pub uninterp spec fn IsValidTcbHashTableEntry(table: UInt64, i: UInt64) -> bool;

pub uninterp spec fn FirstInvalidTcbHashTableEntry(table: UInt64) -> Int64;

pub uninterp spec fn ExceedsMaxTcbHashes(table: UInt64) -> bool;

pub uninterp spec fn TcbHashesLocked() -> bool;

pub uninterp spec fn TcbHashesRecorded(table: UInt64) -> bool;

pub uninterp spec fn SourceOfEntryIgnored(table: UInt64) -> bool;

pub uninterp spec fn NumValidTcbHashEntries() -> Int64;

} // verus!
