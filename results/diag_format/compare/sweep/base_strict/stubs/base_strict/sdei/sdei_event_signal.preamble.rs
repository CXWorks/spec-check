use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type UInt64 = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const NOT_SUPPORTED: Int64 = (-1) as i64;
pub spec const INVALID_PARAMETERS: Int64 = (-2) as i64;
pub spec const SUCCESS: Int64 = 0;

#[allow(non_upper_case_globals)]
pub spec const event: UInt64 = 7;
#[allow(non_upper_case_globals)]
pub spec const target_pe: UInt64 = 13;

pub uninterp spec fn SdeiIsSupported() -> bool;

pub uninterp spec fn ResultEqual(result: Int64, code: Int64) -> bool;

pub uninterp spec fn IsValidMpidr(mpidr: UInt64) -> bool;

pub uninterp spec fn EventIsPending(pe: UInt64, ev: UInt64) -> bool;

pub uninterp spec fn EventIsPrivateTo(pe: UInt64, ev: UInt64) -> bool;

pub uninterp spec fn ClientSharedDataObservableBeforeEvent(pe: UInt64, ev: UInt64) -> bool;

} // verus!
