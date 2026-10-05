use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u64,
}

pub const FFA_ERROR: u32 = 0x84000060u32;
pub const FFA_SUCCESS64: u32 = 0xC4000061u32;

pub const NOT_SUPPORTED: i32 = -1i32;
pub const INVALID_PARAMETERS: i32 = -2i32;
pub const DENIED: i32 = -6i32;
pub const RETRY: i32 = -7i32;
pub const NOT_READY: i32 = -10i32;

pub open spec fn IsFfaPartitionInfoGetRegsImplemented(s: S) -> bool;
pub open spec fn IsValidPartitionUuid(s: S, uuid_lo: u64, uuid_hi: u64) -> bool;
pub open spec fn PartitionInfoCount(s: S, uuid_lo: u64, uuid_hi: u64) -> u64;
pub open spec fn PartitionInfoTag(s: S, uuid_lo: u64, uuid_hi: u64) -> u64;
pub open spec fn IsCalleeNotInStateToHandleRequest(s: S) -> bool;
pub open spec fn IsCalleeNotReady(s: S) -> bool;
pub open spec fn PartitionId(s: S, uuid_lo: u64, uuid_hi: u64, idx: int) -> u64;
pub open spec fn PartitionExecCtxCount(s: S, uuid_lo: u64, uuid_hi: u64, idx: int) -> u64;
pub open spec fn PartitionProperties(s: S, uuid_lo: u64, uuid_hi: u64, idx: int) -> u64;
pub open spec fn PartitionProtocolUuidLo(s: S, uuid_lo: u64, uuid_hi: u64, idx: int) -> u64;
pub open spec fn PartitionProtocolUuidHi(s: S, uuid_lo: u64, uuid_hi: u64, idx: int) -> u64;
pub open spec fn PartitionHasImageUuid(s: S, uuid_lo: u64, uuid_hi: u64, idx: int) -> bool;
pub open spec fn PartitionImageUuidLo(s: S, uuid_lo: u64, uuid_hi: u64, idx: int) -> u64;
pub open spec fn PartitionImageUuidHi(s: S, uuid_lo: u64, uuid_hi: u64, idx: int) -> u64;
pub open spec fn PartitionFfaMajorVersion(s: S, uuid_lo: u64, uuid_hi: u64, idx: int) -> u64;
pub open spec fn PartitionFfaMinorVersion(s: S, uuid_lo: u64, uuid_hi: u64, idx: int) -> u64;

} // verus!
