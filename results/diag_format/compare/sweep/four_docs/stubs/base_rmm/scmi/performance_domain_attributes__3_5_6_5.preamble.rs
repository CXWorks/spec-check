use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub struct BitVec64 {
    pub value: u64,
}

pub trait BitIndexArg {}

impl BitIndexArg for int {}

impl BitIndexArg for core::ops::Range<int> {}

impl BitVec64 {
    pub uninterp spec fn spec_index<T: BitIndexArg>(self, i: T) -> int;
}

pub struct S {
    pub domain_id: u64,
    pub status: StatusCode,
    pub attributes: BitVec64,
    pub rate_limit: BitVec64,
    pub qos_capability_types: BitVec64,
}

pub enum StatusCode {
    SUCCESS,
    FAILURE,
}

pub type RsiCommandReturnCode = u64;

pub spec const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub spec const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub spec const RSI_ERROR_STATE: RsiCommandReturnCode = 2;

pub spec const SUCCESS: StatusCode = StatusCode::SUCCESS;

pub uninterp spec fn IsValidPerformanceDomain(id: u64) -> bool;

pub uninterp spec fn domain_id(s: S) -> u64;

pub uninterp spec fn status(s: S) -> StatusCode;

pub uninterp spec fn attributes(s: S) -> BitVec64;

pub uninterp spec fn rate_limit(s: S) -> BitVec64;

pub uninterp spec fn qos_capability_types(s: S) -> BitVec64;

} // verus!
