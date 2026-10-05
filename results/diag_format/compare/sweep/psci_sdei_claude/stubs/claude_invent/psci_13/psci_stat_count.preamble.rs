use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type UInt32 = u32;

pub struct S {
    pub psci_stat_count: Map<(UInt64, UInt32), UInt64>,
    pub psci_stat_residency: Map<(UInt64, UInt32), UInt64>,
}

pub open spec fn IsValidPsciStatNodeAndState(s: S, target_cpu: UInt64, power_state: UInt32) -> bool;

pub open spec fn PsciStatCountOf(s: S, target_cpu: UInt64, power_state: UInt32) -> UInt64;

} // verus!
