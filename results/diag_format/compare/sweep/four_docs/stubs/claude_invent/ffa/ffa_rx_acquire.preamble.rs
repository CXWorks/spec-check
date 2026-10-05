use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type FfaReturnCode = i32;

pub const FFA_SUCCESS: FfaReturnCode = 0;
pub const NOT_SUPPORTED: FfaReturnCode = -1;
pub const INVALID_PARAMETERS: FfaReturnCode = -2;
pub const DENIED: FfaReturnCode = -6;

pub struct S {
    pub dummy: int,
}

pub open spec fn FfaRxAcquireImplemented(s: S) -> bool;

pub open spec fn RxTxBufferPairRegistered(s: S, vm_id: UInt32) -> bool;

pub open spec fn CalleeCanRelinquishRxBuffer(s: S, vm_id: UInt32) -> bool;

pub open spec fn RxBufferOwnedByCaller(s: S, vm_id: UInt32) -> bool;

} // verus!
