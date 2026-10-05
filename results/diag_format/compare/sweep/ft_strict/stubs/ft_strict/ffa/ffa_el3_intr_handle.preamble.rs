use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt64 = u64;

pub type FfaInstance = u32;
pub type Conduit = u32;

pub type ExceptionLevel = u8;

pub struct S {
    pub dummy: u64,
}

#[allow(non_snake_case)]
pub struct ScrEl3 {
    pub FIQ: u32,
}

pub spec const NOT_SUPPORTED: UInt32 = 0xFFFF_FFFFu32;
pub spec const FFA_SUCCESS: UInt32 = 0x8400_0061u32;

pub spec const S_EL1: ExceptionLevel = 1u8;
pub spec const S_EL2: ExceptionLevel = 2u8;
pub spec const CallerEl: ExceptionLevel = 3u8;

pub spec const SCR_EL3: ScrEl3 = ScrEl3 { FIQ: 0u32 };

pub uninterp spec fn IsSupportedFfaInstance(s: S, ffa_instance: FfaInstance, conduit: Conduit) -> bool;

pub uninterp spec fn ResultEqual(a: UInt32, b: UInt32) -> bool;

pub uninterp spec fn PendingGroup0InterruptHandledByEl3(s: S) -> bool;

pub uninterp spec fn El3SwitchedSecurityState(s: S) -> bool;

pub uninterp spec fn ReturnsToCallingElInSecureState(s: S) -> bool;

} // verus!
