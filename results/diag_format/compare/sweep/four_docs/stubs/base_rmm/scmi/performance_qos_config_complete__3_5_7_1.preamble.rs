use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type UInt64 = u64;

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;

pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;

pub const RSI_INCOMPLETE: RsiCommandReturnCode = 3;

pub struct S {
    pub dummy: int,
}

pub open spec fn ResultEqual(a: RsiCommandReturnCode, b: RsiCommandReturnCode) -> bool;

pub open spec fn AllDomainsQosAtPlatformDefault(capability: UInt32) -> bool;

pub open spec fn DomainAndSiblingsQosAtPlatformDefault(domain_id: UInt32, capability: UInt32) -> bool;

pub open spec fn QosAtPlatformDefault(domain_id: UInt32, capability: UInt32) -> bool;

pub open spec fn QosValue(domain_id: UInt32, capability: UInt32) -> UInt32;

} // verus!
