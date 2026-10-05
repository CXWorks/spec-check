use vstd::prelude::*;
verus! {

#[allow(non_camel_case_types)]
pub enum PsciStatusCode {
    SUCCESS,
    NOT_SUPPORTED,
    INVALID_PARAMETERS,
    DENIED,
    ALREADY_ON,
    ON_PENDING,
    INTERNAL_FAILURE,
    NOT_PRESENT,
    DISABLED,
    INVALID_ADDRESS,
}

#[allow(non_camel_case_types)]
pub enum PsciCoreState {
    ON,
    OFF,
    ON_PENDING,
}

pub struct S {
    pub target_cpu: u64,
    pub entry_point: u64,
    pub context_id: u64,
}

impl S {
    pub open spec fn core_state(self, cpu: u64) -> PsciCoreState;
}

} // verus!
