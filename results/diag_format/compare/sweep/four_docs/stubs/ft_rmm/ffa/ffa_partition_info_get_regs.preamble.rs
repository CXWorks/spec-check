use vstd::prelude::*;

verus! {

pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub type Result = u32;

pub type FunctionId = u32;

pub const FFA_SUCCESS64: Result = 0;
pub const INVALID_PARAMETERS: Result = 1;
pub const NOT_SUPPORTED: Result = 2;
pub const DENIED: Result = 3;
pub const RETRY: Result = 4;
pub const NOT_READY: Result = 5;

pub const FFA_PARTITION_INFO_GET_REGS: FunctionId = 0xC400008B;

pub struct Uuid {
    pub time_low: u32,
    pub time_mid: u16,
    pub time_hi_and_version: u16,
    pub clock_seq: u16,
    pub node: [u8; 5],
}

pub struct PartitionInfoDescriptor {
    pub desc_protocol_uuid: Uuid,
    pub partition_id: u16,
    pub partition_type: u16,
    pub partition_name: [u8; 0],
}

pub struct S {
    pub dummy: int,
}

pub open spec fn IsValidUuid(s: S, uuid_lo: UInt64, uuid_hi: UInt64) -> bool;

pub open spec fn ResultEqual(a: Result, b: Result) -> bool;

pub open spec fn IsValidStartIndex(s: S, uuid_lo: UInt64, uuid_hi: UInt64, start_index: UInt16) -> bool;

pub open spec fn IsImplementedAtInstance(s: S, func: FunctionId) -> bool;

pub open spec fn CalleeCanHandleRequest(s: S) -> bool;

pub open spec fn CalleeInfoTag(s: S, uuid_lo: UInt64, uuid_hi: UInt64) -> UInt16;

pub open spec fn CalleeIsReady(s: S) -> bool;

pub open spec fn NumEntriesReturned() -> int;

pub open spec fn IsNilUuid(s: S, uuid_lo: UInt64, uuid_hi: UInt64) -> bool;

pub open spec fn DescProtocolUuidFieldsAreZero(s: S, partition_info: [PartitionInfoDescriptor; 14]) -> bool;

pub open spec fn UnusedRegistersAreZero(s: S, partition_info: [PartitionInfoDescriptor; 14]) -> bool;

pub open spec fn AllEntriesReturned(s: S) -> bool;

} // verus!
