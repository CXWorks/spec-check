use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub enum RsiCommandReturnCode {
    RSI_SUCCESS,
    RSI_ERROR_INPUT,
    RSI_ERROR_STATE,
    RSI_INCOMPLETE,
}

pub use RsiCommandReturnCode::*;

pub struct S {
    pub dummy: int,
}

pub open spec fn GetQosConfig(s: S, domain_id: UInt32, capability: UInt32) -> UInt32;

} // verus!
