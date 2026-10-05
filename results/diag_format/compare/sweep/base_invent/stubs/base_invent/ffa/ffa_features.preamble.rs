use vstd::prelude::*;
verus! {

pub type RmiStatusCode = u64;

pub const RMI_SUCCESS: RmiStatusCode = 0;
pub const RMI_ERROR_INPUT: RmiStatusCode = 1;

pub struct S {
    pub cmd_input_function_id: u64,
    pub cmd_input_ffa_fid_or_feature_id: u64,
    pub cmd_input_properties: u64,
    pub cmd_input_other_registers: u64,
}

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

} // verus!
