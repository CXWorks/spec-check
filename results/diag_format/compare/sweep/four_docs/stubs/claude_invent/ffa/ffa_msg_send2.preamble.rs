use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type FfaErrorCode = i32;

pub type FfaVmId = u32;

pub struct FfaMsg {
    pub data: Seq<u8>,
}

pub struct S {
    pub dummy: int,
}

pub const NOT_SUPPORTED: FfaErrorCode = -1;
pub const INVALID_PARAMETERS: FfaErrorCode = -2;
pub const NO_MEMORY: FfaErrorCode = -3;
pub const BUSY: FfaErrorCode = -4;
pub const DENIED: FfaErrorCode = -6;

pub open spec fn ResultEqual(result: Result<(), FfaErrorCode>, code: FfaErrorCode) -> bool;

pub open spec fn FfaMsgHeaderReceiverId(s: S, vm_id: UInt32) -> FfaVmId;

pub open spec fn FfaConduitIsSvc(s: S) -> bool;

pub open spec fn FfaIsVirtualInstance(s: S) -> bool;

pub open spec fn FfaIsSecurePhysicalInstance(s: S) -> bool;

pub open spec fn FfaIsNonSecurePhysicalInstance(s: S) -> bool;

pub open spec fn FfaIsValidSenderVmId(s: S, vm_id: UInt32) -> bool;

pub open spec fn FfaMsgHeaderSenderIdValid(s: S, vm_id: UInt32) -> bool;

pub open spec fn FfaMsgHeaderReceiverIdValid(s: S, vm_id: UInt32) -> bool;

pub open spec fn FfaMsgHeaderOffset(s: S, vm_id: UInt32) -> UInt32;

pub open spec fn FfaMsgHeaderSize(s: S, vm_id: UInt32) -> UInt32;

pub open spec fn FfaMsgPayloadFitsTxBuffer(s: S, vm_id: UInt32) -> bool;

pub open spec fn FfaMsgHeaderUuidRecognized(s: S, vm_id: UInt32) -> bool;

pub open spec fn FfaRxBufferFree(s: S, receiver: FfaVmId) -> bool;

pub open spec fn FfaCalleeCanHandleRequest(s: S) -> bool;

pub open spec fn FfaCallerAllowedMsgSend2(s: S) -> bool;

pub open spec fn FfaReceiverSupportsIndirectMessaging(s: S, receiver: FfaVmId) -> bool;

pub open spec fn FfaRxBufferHasSpaceForMsg(s: S, receiver: FfaVmId, vm_id: UInt32) -> bool;

pub open spec fn FfaMsgSend2Implemented(s: S) -> bool;

pub open spec fn FfaRxBufferContainsMsg(s: S, receiver: FfaVmId, msg: FfaMsg) -> bool;

pub open spec fn FfaTxBufferMsg(s: S, vm_id: UInt32) -> FfaMsg;

pub open spec fn FfaRxBufferFullNotificationPending(s: S, receiver: FfaVmId) -> bool;

pub open spec fn FfaScheduleReceiverInterruptSignaled(s: S, receiver: FfaVmId) -> bool;

} // verus!
