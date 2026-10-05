use vstd::prelude::*;

verus! {

pub type UInt128 = u128;
pub type UInt64 = u64;
pub type UInt32 = u32;
pub type Int32 = i32;
pub type PartitionId = u16;

pub struct S {
    pub dummy: u64,
}

pub spec const caller: PartitionId = 1;

pub spec const FFA_PARTITION_INFO_GET: UInt32 = 0x84000068;

pub spec const NOT_SUPPORTED: Int32 = -1;
pub spec const INVALID_PARAMETERS: Int32 = -2;
pub spec const NO_MEMORY: Int32 = -3;
pub spec const BUSY: Int32 = -4;
pub spec const DENIED: Int32 = -6;
pub spec const NOT_READY: Int32 = -10;

pub spec const FFA_SUCCESS: Result<(), Int32> = Ok(());

pub open spec fn RxBufferIsFree(s: S, id: PartitionId) -> bool;

pub open spec fn RxBufferIsMapped(s: S, id: PartitionId) -> bool;

pub open spec fn IsValidUuid(s: S, uuid: UInt128) -> bool;

pub open spec fn ResultsFitInRxBuffer(s: S, id: PartitionId, uuid: UInt128) -> bool;

pub open spec fn CalleeCanHandleRequest(s: S) -> bool;

pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32) -> bool;

pub open spec fn CalleeIsReady(s: S) -> bool;

pub open spec fn ResultEqual(result: Result<(), Int32>, code: Int32) -> bool;

pub open spec fn ReturnedFunction(s: S) -> Result<(), Int32>;

pub open spec fn RxBuffer(s: S, id: PartitionId) -> Seq<u8>;

pub open spec fn PartitionCount(s: S, uuid: UInt128) -> UInt32;

} // verus!
