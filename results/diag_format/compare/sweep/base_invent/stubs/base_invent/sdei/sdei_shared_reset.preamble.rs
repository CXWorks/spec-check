use vstd::prelude::*;
verus! {

pub enum RsiCommandReturnCode {
    RSI_SUCCESS,
    RSI_ERROR_INPUT,
    RSI_ERROR_STATE,
    RSI_INCOMPLETE,
    RSI_ERROR_UNKNOWN,
    RSI_NOT_SUPPORTED,
    RSI_DENIED,
}

pub struct S {
    pub dummy: u64,
}

impl S {
    pub open spec fn sdei_shared_events_running(self) -> bool;
    pub open spec fn sdei_interrupt_bindings_registered(self) -> bool;
    pub open spec fn sdei_supported(self) -> bool;
}

} // verus!
