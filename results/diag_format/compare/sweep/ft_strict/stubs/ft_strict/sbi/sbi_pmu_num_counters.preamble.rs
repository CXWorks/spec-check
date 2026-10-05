use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

pub spec const SBI_SUCCESS: i64 = 0;

pub open spec fn ResultEqual(r: sbiret, code: i64) -> bool;

pub open spec fn NumHardwareCounters() -> int;

pub open spec fn NumFirmwareCounters() -> int;

} // verus!
