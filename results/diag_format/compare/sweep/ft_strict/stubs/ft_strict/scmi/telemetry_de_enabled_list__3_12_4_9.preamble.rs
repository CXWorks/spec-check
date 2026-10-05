use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub enum RsiCommandReturnCode {
    Success,
    OutOfRange,
    Error,
}

pub type Int32 = Result<(), RsiCommandReturnCode>;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());

pub spec const OUT_OF_RANGE: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::OutOfRange);

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn IsValidEnabledListIndex(s: S, index: UInt32, kind: UInt32) -> bool;

pub open spec fn ArrayLength(s: S, array: [UInt32; 1]) -> UInt32;

pub open spec fn RemainingEnabledElements(s: S, index: UInt32, kind: UInt32, count: UInt32) -> UInt32;

pub open spec fn ArrayEntryWord(s: S, array: [UInt32; 1], i: UInt32, word: int) -> UInt32;

pub open spec fn IsEnabledElement(s: S, element: UInt32, kind: UInt32) -> bool;

pub open spec fn IsEnabledWithTimestamps(s: S, element: UInt32, kind: UInt32) -> bool;

} // verus!
