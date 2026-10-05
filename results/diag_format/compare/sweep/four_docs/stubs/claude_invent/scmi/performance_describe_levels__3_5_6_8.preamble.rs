use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -3;

pub open spec fn IsValidPerfDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn PerfLevelCount(s: S, domain_id: UInt32) -> int;

pub open spec fn PerfLevelEntry(s: S, domain_id: UInt32, index: int) -> (UInt32, UInt32, UInt32, UInt32, UInt32);

} // verus!
