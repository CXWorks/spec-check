use vstd::prelude::*;
verus! {

pub type RsiCommandReturnCode = u32;

pub const SUCCESS: RsiCommandReturnCode = 0;
pub const DENIED: RsiCommandReturnCode = 1;
pub const NOT_SUPPORTED: RsiCommandReturnCode = 2;

pub struct SmmuConfiguration {
    pub value: int,
}

pub struct S {
    pub memory_protections_in_place: bool,
    pub smmu_config: SmmuConfiguration,
}

pub open spec fn there_are_memory_protections_in_place(s: S) -> bool;

pub open spec fn SmmuConfig(s: S) -> SmmuConfiguration;

} // verus!
