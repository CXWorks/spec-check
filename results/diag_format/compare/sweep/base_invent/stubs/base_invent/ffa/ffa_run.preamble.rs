use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type UInt16 = u16;

pub struct TargetInfoT {
    pub sp_id: UInt16,
    pub vcpu_id: UInt16,
}

pub struct S {
    pub caller_id: UInt16,
    pub target_sp_id: UInt16,
    pub target_vcpu_id: UInt16,
    pub current_pe: u64,
}

pub const FFA_SUCCESS: int32 = 0;
pub const FFA_ERROR_NOT_SUPPORTED: int32 = -1;
pub const FFA_ERROR_INVALID_PARAMETERS: int32 = -2;
pub const FFA_ERROR_DENIED: int32 = -6;
pub const FFA_ERROR_BUSY: int32 = -4;
pub const FFA_ERROR_ABORTED: int32 = -8;
pub const FFA_ERROR_NOT_READY: int32 = -7;

pub open spec fn TargetInfo(s: S) -> TargetInfoT;

pub open spec fn ResultEqual(result: int32, code: int32) -> bool;

pub open spec fn IsEndpointValid(s: S, sp_id: UInt16) -> bool;

pub open spec fn IsVcpuValid(s: S, vcpu_id: UInt16) -> bool;

pub open spec fn IsVcpuPinnedToDifferentPe(s: S, vcpu_id: UInt16) -> bool;

pub open spec fn IsFfaInstanceValid(s: S) -> bool;

pub open spec fn IsCalleeStateValid(s: S, vcpu_id: UInt16) -> bool;

pub open spec fn IsCallerAllowed(s: S) -> bool;

pub open spec fn IsVcpuBusy(s: S, vcpu_id: UInt16) -> bool;

pub open spec fn IsVcpuAborted(s: S, vcpu_id: UInt16) -> bool;

pub open spec fn IsEndpointReady(s: S, sp_id: UInt16) -> bool;

} // verus!
