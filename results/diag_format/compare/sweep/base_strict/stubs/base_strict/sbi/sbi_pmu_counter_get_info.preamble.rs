use vstd::prelude::*;

verus! {

pub type SbiErrorCode = i64;
pub type CounterInfo = u64;
pub type CounterIdx = u64;

pub spec const SBI_SUCCESS: SbiErrorCode = 0;
pub spec const XLEN: int = 64;

pub struct S {
    pub cmd_input_counter_idx: CounterIdx,
}

pub open spec fn Bits(value: CounterInfo, hi: int, lo: int) -> int;

pub open spec fn CounterTypeEncoding(idx: CounterIdx) -> int;

pub open spec fn IsHardwareCounter(idx: CounterIdx) -> bool;

pub open spec fn IsFirmwareCounter(idx: CounterIdx) -> bool;

pub open spec fn CounterCsrNumber(idx: CounterIdx) -> int;

pub open spec fn CounterBitWidth(idx: CounterIdx) -> int;

} // verus!
