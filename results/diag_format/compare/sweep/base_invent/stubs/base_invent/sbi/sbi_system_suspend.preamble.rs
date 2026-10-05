use vstd::prelude::*;
verus! {

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub hart_state: u64,
    pub a0: u64,
    pub a1: u64,
    pub hartid: int,
    pub opaque: int,
}

pub const HART_STARTED: u64 = 0;
pub const HART_STOPPED: u64 = 1;
pub const HART_START_PENDING: u64 = 2;
pub const HART_STOP_PENDING: u64 = 3;
pub const HART_SUSPENDED: u64 = 4;

} // verus!
