use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Bits1 = u8;
pub type FuncId = u32;
pub type Uuid = u128;

pub enum FfaStatusCode {
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

pub struct FfaMsgHeader {
    pub flags: u32,
    pub reserved: u32,
    pub offset: u32,
    pub sender_id: UInt16,
    pub receiver_id: UInt16,
    pub size: u32,
    pub uuid: Uuid,
}

pub struct S {
    pub dummy: nat,
}

pub spec const FFA_SUCCESS: Result<(), FfaStatusCode> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<(), FfaStatusCode> = Err(FfaStatusCode::InvalidParameters);
pub spec const NO_MEMORY: Result<(), FfaStatusCode> = Err(FfaStatusCode::NoMemory);
pub spec const BUSY: Result<(), FfaStatusCode> = Err(FfaStatusCode::Busy);
pub spec const DENIED: Result<(), FfaStatusCode> = Err(FfaStatusCode::Denied);

pub const FFA_MSG_SEND2: FuncId = 0x84000086;

pub open spec fn ResultEqual(r1: Result<(), FfaStatusCode>, r2: Result<(), FfaStatusCode>) -> bool;
pub open spec fn IsNonSecurePhysicalInstance(s: S) -> bool;
pub open spec fn IsValidSenderVmId(s: S, id: UInt16) -> bool;
pub open spec fn IsValidEndpointId(s: S, id: UInt16) -> bool;
pub open spec fn MsgHeader(s: S) -> FfaMsgHeader;
pub open spec fn MsgHeaderSize(s: S) -> u32;
pub open spec fn MsgPayloadFitsInTxBuffer(s: S, h: FfaMsgHeader) -> bool;
pub open spec fn IsRecognizedUuid(s: S, uuid: Uuid) -> bool;
pub open spec fn RxBufferIsFree(s: S, id: UInt16) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S) -> bool;
pub open spec fn CallerMayInvoke(s: S, f: FuncId) -> bool;
pub open spec fn SupportsIndirectMessaging(s: S, id: UInt16) -> bool;
pub open spec fn RxBufferHasSpaceFor(s: S, id: UInt16, h: FfaMsgHeader) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, f: FuncId) -> bool;
pub open spec fn SenderTxBuffer(s: S) -> Seq<u8>;
pub open spec fn PartitionMessage(buf: Seq<u8>) -> Seq<u8>;
pub open spec fn RxBufferContains(s: S, id: UInt16, msg: Seq<u8>) -> bool;
pub open spec fn RxBufferFullNotified(s: S, id: UInt16) -> bool;
pub open spec fn RxBuffer(s: S, id: UInt16) -> Seq<u8>;
pub open spec fn RxBufferFullNotification(s: S, id: UInt16) -> bool;

} // verus!
