use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt16 = u16;

pub struct S {
    pub dummy: int,
}

pub const FFA_ERROR: UInt32 = 0x84000060u32;
pub const FFA_SUCCESS: UInt32 = 0x84000061u32;
pub const NOT_SUPPORTED: UInt32 = 0xFFFFFFFFu32;

pub open spec fn FfaSpmIdGetImplementedAtInstance(s: S) -> bool;
pub open spec fn FfaInstanceIsNonSecurePhysical(s: S) -> bool;
pub open spec fn FfaInstanceIsNonSecureVirtual(s: S) -> bool;
pub open spec fn FfaInstanceIsSecureVirtual(s: S) -> bool;
pub open spec fn FfaInstanceIsSecurePhysical(s: S) -> bool;
pub open spec fn SpmcImplementedAtEl3(s: S) -> bool;
pub open spec fn SpmcId(s: S) -> UInt16;
pub open spec fn SpmdId(s: S) -> UInt16;

} // verus!
