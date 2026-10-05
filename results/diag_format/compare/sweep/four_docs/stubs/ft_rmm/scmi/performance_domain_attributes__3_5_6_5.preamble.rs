#![allow(overflowing_literals)]
use vstd::prelude::*;
verus! {

pub type uint8 = u8;
pub type uint32 = i32;
pub type int32 = i32;

pub enum RsiCommandReturnCode {
    ErrorInput,
    ErrorState,
    ErrorNotFound,
    ErrorUnknown,
}

pub struct S {
    pub num_performance_domains: nat,
}

pub spec const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());
pub spec const RSI_ERROR_NOT_FOUND: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::ErrorNotFound);

pub uninterp spec fn IsValidPerformanceDomain(s: S, domain_id: uint32) -> bool;

} // verus!
