use vstd::prelude::*;
use vstd::arithmetic::power2::pow2;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Mpidr = u64;
pub type PowerLevel = u32;
pub type LocalState = u32;

pub struct PowerState {
    pub raw: u32,
}

pub struct PwrNode {
    pub id: nat,
}

pub struct S {
    pub dummy: int,
}

pub const NOT_SUPPORTED: UInt64 = 0xFFFF_FFFF_FFFF_FFFFu64;

pub open spec fn StatFunctionsImplemented() -> bool;

pub open spec fn ResultEqual(result: UInt64, expected: UInt64) -> bool;

pub open spec fn IsPresentNode(target_cpu: Mpidr) -> bool;

pub open spec fn NodeSupportsState(node: PwrNode, state: LocalState) -> bool;

pub open spec fn StatNode(target_cpu: Mpidr, power_state: PowerState) -> PwrNode;

pub open spec fn StatLocalState(power_state: PowerState) -> LocalState;

pub open spec fn NodeAt(target_cpu: Mpidr, level: PowerLevel) -> PwrNode;

pub open spec fn HighestPowerLevel(power_state: PowerState) -> PowerLevel;

pub open spec fn IsStatCount(fid: UInt32) -> bool;

pub open spec fn IsStatResidency(fid: UInt32) -> bool;

pub open spec fn StateUseCount(node: PwrNode, state: LocalState) -> nat;

pub open spec fn StateResidencyUs(node: PwrNode, state: LocalState) -> nat;

pub open spec fn ResultBits(fid: UInt32) -> u32;

} // verus!
