use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type UInt32 = u32;
pub type PsciReturnCode = i32;

pub const ON: PsciReturnCode = 0;
pub const OFF: PsciReturnCode = 1;
pub const ON_PENDING: PsciReturnCode = 2;
pub const INVALID_PARAMETERS: PsciReturnCode = -2;
pub const DISABLED: PsciReturnCode = -8;

pub struct S {
    pub dummy: int,
}

pub open spec fn AffinityInstanceIsPresent(s: S, target_affinity: UInt64, lowest_affinity_level: int) -> bool;
pub open spec fn AffinityInstanceIsDisabled(s: S, target_affinity: UInt64, lowest_affinity_level: int) -> bool;
pub open spec fn AffinityInstanceAnyCoreOn(s: S, target_affinity: UInt64, lowest_affinity_level: int) -> bool;
pub open spec fn AffinityInstanceAnyCoreOnPending(s: S, target_affinity: UInt64, lowest_affinity_level: int) -> bool;
pub open spec fn AffinityInstanceAllCoresOff(s: S, target_affinity: UInt64, lowest_affinity_level: int) -> bool;

} // verus!
