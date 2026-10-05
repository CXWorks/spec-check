use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type FfaReturnStatus = i32;
pub type PartitionId = u16;
pub type FunctionId = u32;
pub type Uuid = u128;
pub type Instance = u8;

pub struct S {
    pub caller: PartitionId,
    pub callee: PartitionId,
    pub cmd_input_fid: FunctionId,
    pub cmd_input_uuid: Uuid,
    pub cmd_input_flags: UInt32,
    pub instance: Instance,
}

pub spec const FFA_SUCCESS: FfaReturnStatus = 0;
pub spec const NOT_SUPPORTED: FfaReturnStatus = (-1int) as FfaReturnStatus;
pub spec const INVALID_PARAMETERS: FfaReturnStatus = (-2int) as FfaReturnStatus;
pub spec const NO_MEMORY: FfaReturnStatus = (-3int) as FfaReturnStatus;
pub spec const BUSY: FfaReturnStatus = (-4int) as FfaReturnStatus;
pub spec const DENIED: FfaReturnStatus = (-6int) as FfaReturnStatus;
pub spec const NOT_READY: FfaReturnStatus = (-9int) as FfaReturnStatus;

pub spec const FFA_PARTITION_INFO_GET: FunctionId = 0x84000068;

pub uninterp spec fn Bits64(value: int, hi: int, lo: int) -> int;

pub uninterp spec fn RxBufferFree(id: PartitionId) -> bool;

pub uninterp spec fn RxBufferMapped(id: PartitionId) -> bool;

pub uninterp spec fn ResultEqual(result: FfaReturnStatus, expected: FfaReturnStatus) -> bool;

pub uninterp spec fn IsValidUuid(uuid: Uuid) -> bool;

pub uninterp spec fn PartitionInfoFitsInRxBuffer(id: PartitionId, uuid: Uuid) -> bool;

pub uninterp spec fn CalleeInStateToHandleRequest(callee: PartitionId) -> bool;

pub uninterp spec fn IsImplementedAtInstance(fid: FunctionId, instance: Instance) -> bool;

pub uninterp spec fn CalleeReadyToHandleRequest(callee: PartitionId) -> bool;

pub uninterp spec fn PartitionInfoDescriptorCount(uuid: Uuid) -> UInt32;

pub uninterp spec fn RxBufferHoldsPartitionInfoDescriptors(id: PartitionId, uuid: Uuid, count: UInt32, size: UInt32) -> bool;

pub uninterp spec fn PartitionInfoDescriptorSize(uuid: Uuid) -> UInt32;

pub uninterp spec fn DeployedPartitionCount(uuid: Uuid) -> UInt32;

} // verus!
