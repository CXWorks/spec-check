use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub enum FfaFunction {
    FfaRxRelease,
    FfaOther,
}

pub type FfaInstance = int;

pub type RxBuffer = int;

pub const FFA_RX_RELEASE: FfaFunction = FfaFunction::FfaRxRelease;

pub const FFA_SUCCESS: UInt32 = 0;
pub const NOT_SUPPORTED: UInt32 = 1;
pub const INVALID_PARAMETERS: UInt32 = 2;
pub const DENIED: UInt32 = 3;

pub open spec fn IsImplementedAtInstance(s: S, f: FfaFunction, inst: FfaInstance) -> bool;

pub open spec fn CurrentFfaInstance(s: S) -> FfaInstance;

pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;

pub open spec fn IsRxTxPairRegisteredByHypervisor(s: S, vm_id: UInt16) -> bool;

pub open spec fn CallerOwnsRxBuffer(s: S, buf: RxBuffer) -> bool;

pub open spec fn TargetRxBuffer(s: S, vm_id: UInt16) -> RxBuffer;

} // verus!
