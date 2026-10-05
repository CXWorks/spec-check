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

pub struct PowerCapDomainState {
    pub power_cap: UInt32,
    pub cpli: UInt32,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn PowerCapDomain(s: S, domain_id: UInt32) -> PowerCapDomainState;

pub open spec fn DomainSupportsCpc(s: S, domain_id: UInt32) -> bool;

} // verus!
