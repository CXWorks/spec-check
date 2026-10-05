use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type RsiCommandReturnCode = u32;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_NOT_FOUND: RsiCommandReturnCode = 1;
pub const RSI_NOT_SUPPORTED: RsiCommandReturnCode = 2;
pub const RSI_INVALID_PARAMETERS: RsiCommandReturnCode = 3;
pub const RSI_DENIED: RsiCommandReturnCode = 4;

pub struct PowerCapDomain {
    pub power_cap: UInt32,
}

pub struct S {
    pub domains: Map<UInt32, PowerCapDomain>,
}

pub open spec fn PowerCapDomainAt(s: S, domain_id: UInt32) -> PowerCapDomain;

} // verus!
