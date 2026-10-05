use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type FfaStatus = i32;

pub const FFA_SUCCESS: FfaStatus = 0;
pub const NOT_SUPPORTED: FfaStatus = -1;
pub const INVALID_PARAMETERS: FfaStatus = -2;
pub const DENIED: FfaStatus = -6;

pub struct S {
    pub placeholder: int,
}

pub open spec fn FfaRxReleaseImplemented(s: S) -> bool;

pub open spec fn IsNonSecurePhysicalInstance(s: S) -> bool;

pub open spec fn BufferPairRegisteredByHypervisorForVm(s: S, vm_id: UInt32) -> bool;

pub open spec fn CallerOwnsRxBuffer(s: S, vm_id: UInt32) -> bool;

pub open spec fn RxBufferOwnershipRelinquished(old_s: S, new_s: S, vm_id: UInt32) -> bool;

} // verus!
