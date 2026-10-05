use vstd::prelude::*;
verus! {

pub struct S {
    pub source_endpoint_id: int,
    pub destination_endpoint_id: int,
}

pub spec const FFA_SUCCESS: int = 0;
pub spec const FFA_ERROR_NOT_SUPPORTED: int = -1;
pub spec const FFA_ERROR_INVALID_PARAMETERS: int = -2;
pub spec const FFA_ERROR_DENIED: int = -6;
pub spec const FFA_ERROR_ABORTED: int = -8;

} // verus!
