use vstd::prelude::*;

verus! {

pub type ErrorCode = i32;

pub const NOT_SUPPORTED: ErrorCode = -1;
pub const INVALID_PARAMETERS: ErrorCode = -2;
pub const NO_MEMORY: ErrorCode = -3;
pub const BUSY: ErrorCode = -4;
pub const DENIED: ErrorCode = -6;
pub const NOT_READY: ErrorCode = -9;

pub struct FfaResult {
    pub func_id: u32,
    pub error_code: i32,
}

pub struct S {
    pub version: u32,
}

pub open spec fn FfaPartitionInfoGetImplemented(s: S) -> bool;

pub open spec fn IsValidPartitionUuid(s: S, uuid: u128) -> bool;

pub open spec fn RxBufferFreeAndMapped(s: S) -> bool;

pub open spec fn PartitionInfoFitsInRxBuffer(s: S, uuid: u128) -> bool;

pub open spec fn CalleeInStateToHandleRequest(s: S) -> bool;

pub open spec fn CalleeReadyToHandleRequest(s: S) -> bool;

pub open spec fn FfaResultIsError(r: FfaResult) -> bool;

pub open spec fn FfaResultIsErrorCode(r: FfaResult, code: ErrorCode) -> bool;

pub open spec fn FfaResultIsSuccess(r: FfaResult) -> bool;

pub open spec fn PartitionCountForUuid(s: S, uuid: u128) -> int;

pub open spec fn PartitionInfoDescriptorSize(s: S) -> int;

pub open spec fn RxBufferHoldsPartitionInfoDescriptors(s: S, uuid: u128, count: int, size: int) -> bool;

pub open spec fn RxBufferOwnedByCaller(s: S) -> bool;

} // verus!
