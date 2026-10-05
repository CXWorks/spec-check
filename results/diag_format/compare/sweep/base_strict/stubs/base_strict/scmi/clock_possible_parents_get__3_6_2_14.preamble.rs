use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct Array<T> {
    pub data: Seq<T>,
}

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = 1;
pub spec const OUT_OF_RANGE: Int32 = 2;
pub spec const NOT_SUPPORTED: Int32 = 3;
pub spec const DENIED: Int32 = 4;

pub spec const clock_id: UInt32 = 1001;
pub spec const skip_parents: UInt32 = 1002;
pub spec const calling_agent: UInt32 = 1003;

pub uninterp spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub uninterp spec fn ClockExists(s: S, clock: UInt32) -> bool;

pub uninterp spec fn IsValidSkipParents(s: S, clock: UInt32, skip: UInt32) -> bool;

pub uninterp spec fn PossibleParentsAdvertisementSupported(s: S, clock: UInt32) -> bool;

pub uninterp spec fn AgentMayGetPossibleParents(s: S, agent: UInt32, clock: UInt32) -> bool;

pub uninterp spec fn Bits64(value: UInt32, hi: int, lo: int) -> UInt32;

pub uninterp spec fn ArrayLength<T>(a: Array<T>) -> UInt32;

pub uninterp spec fn RemainingPossibleParents(s: S, clock: UInt32, skip: UInt32, count: UInt32) -> UInt32;

pub uninterp spec fn ElementAt(a: Array<UInt32>, i: int) -> UInt32;

pub uninterp spec fn NthPossibleParent(s: S, clock: UInt32, n: int) -> UInt32;

} // verus!
