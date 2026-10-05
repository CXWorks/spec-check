use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;

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

pub struct S {
    pub dummy: int,
}

pub spec const FFA_RXTX_UNMAP: UInt32 = 0x84000067u32;

pub spec const FFA_SUCCESS: Result<(), FfaStatusCode> = Ok(());

pub spec const NOT_SUPPORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::NotSupported);

pub spec const INVALID_PARAMETERS: Result<(), FfaStatusCode> = Err(FfaStatusCode::InvalidParameters);

pub uninterp spec fn IsImplementedAtInstance(s: S, func_id: UInt32) -> bool;

pub uninterp spec fn ResultEqual(r1: Result<(), FfaStatusCode>, r2: Result<(), FfaStatusCode>) -> bool;

pub uninterp spec fn BufferOwner(s: S, id: UInt16) -> UInt16;

pub uninterp spec fn IsRxTxBufferPairRegistered(s: S, owner: UInt16) -> bool;

pub uninterp spec fn IsRxTxBufferPairMappedInCalleeRegime(s: S, owner: UInt16) -> bool;

} // verus!
