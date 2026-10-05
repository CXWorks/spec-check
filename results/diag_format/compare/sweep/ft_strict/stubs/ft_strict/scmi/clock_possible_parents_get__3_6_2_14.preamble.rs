use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct CallingAgent {
    pub id: u32,
}

pub struct S {
    pub dummy: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;
pub const OUT_OF_RANGE: Int32 = -2;
pub const NOT_SUPPORTED: Int32 = -3;
pub const DENIED: Int32 = -4;
#[allow(non_upper_case_globals)]
pub const result: Int32 = -5;

pub open spec fn ClockExists(s: S, clock_id: UInt32) -> bool;

pub open spec fn IsValidSkipParents(s: S, clock_id: UInt32, skip_parents: UInt32) -> bool;

pub open spec fn PossibleParentsAdvertisementSupported(s: S, clock_id: UInt32) -> bool;

pub open spec fn AgentMayGetPossibleParents(s: S, calling_agent: CallingAgent, clock_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn ArrayLength(a: [UInt32; 1]) -> UInt32;

pub open spec fn RemainingPossibleParents(s: S, clock_id: UInt32, skip_parents: UInt32, count: int) -> UInt32;

pub open spec fn ElementAt<T>(a: [UInt32; 1], i: T) -> UInt32;

pub open spec fn NthPossibleParent(s: S, clock_id: UInt32, n: int) -> UInt32;

} // verus!
