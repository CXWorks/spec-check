use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type PartitionId = u32;
pub type FfaFunctionId = u32;
pub type Uuid = u128;
pub type BufferAddr = u64;

pub enum FfaCommandReturnCode {
    NotSupported,
    InvalidParameters,
    NoMemory,
    Busy,
    Interrupted,
    Denied,
    Retry,
    Aborted,
    NoData,
}

pub struct S {
    pub instance_kind: u32,
    pub tick: nat,
}

pub spec const caller: PartitionId = 1;

pub spec const FFA_MSG_SEND2: FfaFunctionId = 0x84000086;

pub spec const FFA_SUCCESS: Result<(), FfaCommandReturnCode> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), FfaCommandReturnCode> = Err(FfaCommandReturnCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<(), FfaCommandReturnCode> = Err(FfaCommandReturnCode::InvalidParameters);
pub spec const NO_MEMORY: Result<(), FfaCommandReturnCode> = Err(FfaCommandReturnCode::NoMemory);
pub spec const BUSY: Result<(), FfaCommandReturnCode> = Err(FfaCommandReturnCode::Busy);
pub spec const DENIED: Result<(), FfaCommandReturnCode> = Err(FfaCommandReturnCode::Denied);

pub open spec fn ResultEqual(r1: Result<(), FfaCommandReturnCode>, r2: Result<(), FfaCommandReturnCode>) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, func: FfaFunctionId) -> bool;
pub open spec fn IsVirtualInstance(s: S) -> bool;
pub open spec fn IsSecurePhysicalInstance(s: S) -> bool;
pub open spec fn IsNonSecurePhysicalInstance(s: S) -> bool;
pub open spec fn Bits(v: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn IsValidSenderVmId(s: S, vm_id: UInt32) -> bool;
pub open spec fn MsgSender(s: S, c: PartitionId, sender_vm_id: UInt32) -> PartitionId;
pub open spec fn MsgReceiver(s: S, c: PartitionId, sender_vm_id: UInt32) -> PartitionId;
pub open spec fn IsValidSenderId(s: S, id: PartitionId) -> bool;
pub open spec fn IsValidReceiverId(s: S, id: PartitionId) -> bool;
pub open spec fn MsgOffset(s: S, c: PartitionId, sender_vm_id: UInt32) -> int;
pub open spec fn PartitionMsgHeaderSize(s: S) -> int;
pub open spec fn MsgPayloadFitsInTxBuffer(s: S, c: PartitionId, sender_vm_id: UInt32) -> bool;
pub open spec fn MsgUuid(s: S, c: PartitionId, sender_vm_id: UInt32) -> Uuid;
pub open spec fn IsRecognizedUuid(s: S, uuid: Uuid) -> bool;
pub open spec fn RxBufferIsFree(s: S, receiver: PartitionId) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S) -> bool;
pub open spec fn CallerMayInvoke(s: S, c: PartitionId, func: FfaFunctionId) -> bool;
pub open spec fn SupportsIndirectMessaging(s: S, receiver: PartitionId) -> bool;
pub open spec fn RxBufferHasSpaceForMsg(s: S, receiver: PartitionId, c: PartitionId, sender_vm_id: UInt32) -> bool;
pub open spec fn SourceTxBuffer(s: S, c: PartitionId, sender_vm_id: UInt32) -> BufferAddr;
pub open spec fn RxBufferHoldsMsg(s: S, receiver: PartitionId, src: BufferAddr) -> bool;
pub open spec fn ReceiverSchedulerNotifiedToRun(s: S, receiver: PartitionId) -> bool;

} // verus!
