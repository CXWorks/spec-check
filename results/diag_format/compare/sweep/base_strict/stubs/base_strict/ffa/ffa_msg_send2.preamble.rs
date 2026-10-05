use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u64,
}

pub const FFA_MSG_SEND2: u32 = 0x84000086u32;

pub const FFA_SUCCESS: u32 = 0u32;
pub const NOT_SUPPORTED: u32 = 1u32;
pub const INVALID_PARAMETERS: u32 = 2u32;
pub const NO_MEMORY: u32 = 3u32;
pub const BUSY: u32 = 4u32;
pub const DENIED: u32 = 6u32;

pub open spec fn IsImplementedAtInstance(func_id: u32) -> bool;
pub open spec fn ResultEqual(result: u32, code: u32) -> bool;
pub open spec fn IsVirtualInstance() -> bool;
pub open spec fn IsSecurePhysicalInstance() -> bool;
pub open spec fn IsNonSecurePhysicalInstance() -> bool;
pub open spec fn sender_vm_id(s: S) -> u32;
pub open spec fn IsValidSenderVmId(id: u32) -> bool;
pub open spec fn Bits(value: u32, hi: int, lo: int) -> u32;
pub open spec fn IsValidSenderId(id: u32) -> bool;
pub open spec fn IsValidReceiverId(id: u32) -> bool;
pub open spec fn MsgSender(s: S, vm_id: u32) -> u32;
pub open spec fn MsgReceiver(s: S, vm_id: u32) -> u32;
pub open spec fn MsgOffset(s: S, vm_id: u32) -> u32;
pub open spec fn PartitionMsgHeaderSize() -> u32;
pub open spec fn MsgPayloadFitsInTxBuffer(s: S, vm_id: u32) -> bool;
pub open spec fn MsgUuid(s: S, vm_id: u32) -> u128;
pub open spec fn IsRecognizedUuid(uuid: u128) -> bool;
pub open spec fn RxBufferIsFree(receiver: u32) -> bool;
pub open spec fn CalleeCanHandleRequest() -> bool;
pub open spec fn CallerMayInvoke(s: S, func_id: u32) -> bool;
pub open spec fn SupportsIndirectMessaging(receiver: u32) -> bool;
pub open spec fn RxBufferHasSpaceForMsg(receiver: u32, s: S, vm_id: u32) -> bool;
pub open spec fn SourceTxBuffer(s: S, vm_id: u32) -> Seq<u8>;
pub open spec fn RxBufferHoldsMsg(receiver: u32, msg: Seq<u8>) -> bool;
pub open spec fn ReceiverSchedulerNotifiedToRun(receiver: u32) -> bool;

} // verus!
