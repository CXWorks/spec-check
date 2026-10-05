use vstd::prelude::*;

verus! {

pub type unsigned_long = u64;

pub enum SbiStatusCode {
    Failed,
    NotSupported,
    InvalidParam,
    Denied,
    InvalidAddress,
    AlreadyAvailable,
    AlreadyStarted,
    AlreadyStopped,
}

pub struct S {
    pub vmid: u64,
}

pub spec const SBI_SUCCESS: Result<(), SbiStatusCode> = Ok(());

pub open spec fn CurrentVmid(s: S) -> u64;

pub open spec fn RemoteHartsExecutedHfenceVvma(s: S, hart_mask: u64, hart_mask_base: u64, start_addr: u64, end_addr: int, asid: u64, vmid: u64) -> bool;

} // verus!
