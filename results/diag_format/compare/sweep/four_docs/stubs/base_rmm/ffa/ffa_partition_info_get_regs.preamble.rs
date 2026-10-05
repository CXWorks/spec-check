use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int32 = i32;

pub struct S {
    pub dummy: u64,
}

pub const uuid_lo: UInt64 = 0x1111;
pub const uuid_hi: UInt64 = 0x2222;
pub const start_index: UInt16 = 1;
pub const tag: UInt16 = 2;

pub const FFA_SUCCESS64: UInt32 = 0xC4000061;
pub const INVALID_PARAMETERS: UInt32 = 0xFFFFFFFD;
pub const NOT_SUPPORTED: UInt32 = 0xFFFFFFFF;
pub const DENIED: UInt32 = 0xFFFFFFFA;
pub const RETRY: UInt32 = 0xFFFFFFF7;
pub const NOT_READY: UInt32 = 0xFFFFFFF6;
pub const FFA_PARTITION_INFO_GET_REGS: UInt32 = 0xC400008B;

pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;
pub open spec fn IsValidUuid(s: S, lo: UInt64, hi: UInt64) -> bool;
pub open spec fn IsValidStartIndex(s: S, lo: UInt64, hi: UInt64, idx: UInt16) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S) -> bool;
pub open spec fn CalleeInfoTag(s: S, lo: UInt64, hi: UInt64) -> UInt16;
pub open spec fn CalleeIsReady(s: S) -> bool;
pub open spec fn NumEntriesReturned(s: S, last_index: UInt16, current_index: UInt16, start: UInt16) -> int;
pub open spec fn IsNilUuid(s: S, lo: UInt64, hi: UInt64) -> bool;
pub open spec fn DescProtocolUuidFieldsAreZero(s: S, partition_info: [UInt64; 15]) -> bool;
pub open spec fn UnusedRegistersAreZero(s: S, partition_info: [UInt64; 15]) -> bool;
pub open spec fn AllEntriesReturned(s: S, partition_info: [UInt64; 15]) -> bool;

} // verus!
