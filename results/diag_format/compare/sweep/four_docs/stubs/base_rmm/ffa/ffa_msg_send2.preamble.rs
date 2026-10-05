use vstd::prelude::*;
verus! {

pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type UInt128 = u128;

pub type EndpointId = UInt16;
pub type FfaFunctionId = UInt32;
pub type Uuid = UInt128;

pub struct S {
    pub regs: Seq<UInt64>,
    pub tx_buffer: Seq<UInt8>,
}

pub struct FfaMsgHeader {
    pub flags: UInt32,
    pub offset: UInt32,
    pub sender_id: EndpointId,
    pub receiver_id: EndpointId,
    pub uuid: Uuid,
    pub size: UInt32,
}

pub struct TxBuffer {
    pub data: Seq<UInt8>,
}

pub struct PartitionMsg {
    pub header: FfaMsgHeader,
    pub payload: Seq<UInt8>,
}

pub const FFA_SUCCESS: UInt32 = 0;
pub const FFA_ERROR_NOT_SUPPORTED: UInt32 = 1;
pub const FFA_ERROR_INVALID_PARAMETERS: UInt32 = 2;
pub const FFA_ERROR_NO_MEMORY: UInt32 = 3;
pub const FFA_ERROR_BUSY: UInt32 = 4;
pub const FFA_ERROR_DENIED: UInt32 = 6;

pub const FFA_MSG_SEND2: FfaFunctionId = 0x84000086;

pub open spec fn IsNonSecurePhysicalInstance() -> bool;
pub open spec fn w1(s: S) -> UInt32;
pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;
pub open spec fn IsValidSenderVmId(id: UInt32) -> bool;
pub open spec fn IsValidEndpointId(id: EndpointId) -> bool;
pub open spec fn MsgHeader(s: S) -> FfaMsgHeader;
pub open spec fn MsgHeaderSize() -> UInt32;
pub open spec fn MsgPayloadFitsInTxBuffer(h: FfaMsgHeader) -> bool;
pub open spec fn IsRecognizedUuid(u: Uuid) -> bool;
pub open spec fn RxBufferIsFree(id: EndpointId) -> bool;
pub open spec fn CalleeCanHandleRequest() -> bool;
pub open spec fn CallerMayInvoke(f: FfaFunctionId) -> bool;
pub open spec fn SupportsIndirectMessaging(id: EndpointId) -> bool;
pub open spec fn RxBufferHasSpaceFor(id: EndpointId, h: FfaMsgHeader) -> bool;
pub open spec fn IsImplementedAtInstance(f: FfaFunctionId) -> bool;
pub open spec fn SenderTxBuffer(s: S) -> TxBuffer;
pub open spec fn PartitionMessage(b: TxBuffer) -> PartitionMsg;
pub open spec fn RxBufferContains(s: S, id: EndpointId, m: PartitionMsg) -> bool;
pub open spec fn RxBufferFullNotified(s: S, id: EndpointId) -> bool;

} // verus!
