use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub const XLEN: u64 = 64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
    pub minor: u64,
    pub major: u64,
    pub reserved: u64,
    pub reserved_hi: u64,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn CurrentSbiSpecMinorVersion() -> u64;

pub open spec fn CurrentSbiSpecMajorVersion() -> u64;

} // verus!
