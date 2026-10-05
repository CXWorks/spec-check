use vstd::prelude::*;
verus! {

pub type RsiCommandReturnCode = u64;

pub spec const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub spec const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub spec const RSI_ERROR_STATE: RsiCommandReturnCode = 2;
pub spec const RSI_INCOMPLETE: RsiCommandReturnCode = 3;

pub type InvokerId = u64;

pub type FfaResults = u64;

#[allow(non_upper_case_globals)]
pub spec const results: FfaResults = 0;

pub struct S {
    pub invoker: InvokerId,
    pub pending_results: FfaResults,
}

pub open spec fn PreviousInvoker() -> InvokerId;

pub open spec fn ResultsDeliveredTo(invoker: InvokerId, r: FfaResults) -> bool;

} // verus!
