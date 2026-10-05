use vstd::prelude::*;
verus! {

pub type UInt128 = u128;
pub type UInt32 = u32;
pub type UInt16 = u16;
pub type FfaReturnStatus = i32;
pub type PartitionId = u16;
pub type FfaFunctionId = u32;
pub type InstanceId = u32;

pub struct S {
    pub dummy: u64,
}

pub const FFA_SUCCESS: FfaReturnStatus = 0;
pub const NOT_SUPPORTED: FfaReturnStatus = -1;
pub const INVALID_PARAMETERS: FfaReturnStatus = -2;
pub const NO_MEMORY: FfaReturnStatus = -3;
pub const BUSY: FfaReturnStatus = -4;
pub const DENIED: FfaReturnStatus = -6;
pub const NOT_READY: FfaReturnStatus = -9;

pub const FFA_PARTITION_INFO_GET: FfaFunctionId = 0x84000068;

#[allow(non_upper_case_globals)]
pub const caller: PartitionId = 1;
#[allow(non_upper_case_globals)]
pub const callee: PartitionId = 2;
#[allow(non_upper_case_globals)]
pub const instance: InstanceId = 3;

pub open spec fn ResultEqual(result: FfaReturnStatus, expected: FfaReturnStatus) -> bool;
pub open spec fn RxBufferFree(s: S, id: PartitionId) -> bool;
pub open spec fn RxBufferMapped(s: S, id: PartitionId) -> bool;
pub open spec fn IsValidUuid(s: S, uuid: UInt128) -> bool;
pub open spec fn PartitionInfoFitsInRxBuffer(s: S, id: PartitionId, uuid: UInt128) -> bool;
pub open spec fn CalleeInStateToHandleRequest(s: S, id: PartitionId) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, func: FfaFunctionId, inst: InstanceId) -> bool;
pub open spec fn CalleeReadyToHandleRequest(s: S, id: PartitionId) -> bool;
pub open spec fn PartitionInfoDescriptorCount(s: S, uuid: UInt128) -> UInt32;
pub open spec fn RxBufferHoldsPartitionInfoDescriptors(s: S, id: PartitionId, uuid: UInt128, count: UInt32, size: UInt32) -> bool;
pub open spec fn PartitionInfoDescriptorSize(s: S, uuid: UInt128) -> UInt32;
pub open spec fn DeployedPartitionCount(s: S, uuid: UInt128) -> UInt32;

} // verus!
