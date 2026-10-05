use vstd::prelude::*;
verus! {

pub type sbiret = i64;

pub const SBI_SUCCESS: i64 = 0;

pub struct S {
    pub dummy: u64,
}

pub open spec fn ResultEqual(error: sbiret, code: i64) -> bool;

pub open spec fn NumHardwareCounters() -> u64;

pub open spec fn NumFirmwareCounters() -> u64;

} // verus!
