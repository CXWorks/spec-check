use vstd::prelude::*;
verus! {

pub type SbiError = i64;

pub const SBI_SUCCESS: SbiError = 0;

pub struct sbiret {
    pub error: SbiError,
    pub value: i64,
}

pub struct S {
    pub sie_stie: bool,
    pub sie_timer_pending: bool,
}

} // verus!
