use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub mvendorid: u64,
    pub marchid: u64,
    pub mimpid: u64,
}

pub const XLEN: u64 = 64;

pub open spec fn CurrentSbiSpecMinorVersion() -> u64;

pub open spec fn CurrentSbiSpecMajorVersion() -> u64;

} // verus!
