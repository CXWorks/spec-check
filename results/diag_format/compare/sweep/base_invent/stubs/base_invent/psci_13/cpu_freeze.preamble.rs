use vstd::prelude::*;
verus! {

pub type PsciReturnCode = i32;

pub const PSCI_SUCCESS: PsciReturnCode = 0;
pub const PSCI_NOT_SUPPORTED: PsciReturnCode = -1;
pub const PSCI_DENIED: PsciReturnCode = -3;

pub type PsciFeature = u32;

pub const PSCI_FEATURE_CPU_FREEZE: PsciFeature = 0x8400000B;

pub struct PsciState {
    pub features: Set<u32>,
    pub denied: bool,
}

pub open spec fn PsciFeatureEnabled(s: PsciState, f: PsciFeature) -> bool;

pub open spec fn PsciDenied(s: PsciState) -> bool;

} // verus!
