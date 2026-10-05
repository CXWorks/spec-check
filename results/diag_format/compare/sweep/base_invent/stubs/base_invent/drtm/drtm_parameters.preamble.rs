use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub enum RmiStatusCode {
    RMI_SUCCESS,
    RMI_ERROR_INPUT,
    RMI_ERROR_REALM,
    RMI_ERROR_REC,
    RMI_ERROR_RTT,
    RMI_ERROR_DEVICE,
    RMI_BUSY,
}

pub struct S {
    pub dummy: u64,
}

pub const DRTM_PARAMETERS_SIZE: u64 = 80;
pub const GRANULE_SIZE_4KB: u64 = 4096;

} // verus!
