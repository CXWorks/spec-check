use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type RsiCommandReturnCode = u32;

pub struct S {
    pub dummy: int,
}

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn u32_bits<I>(x: u32, i: I) -> u32;

pub trait BitIndex {
    spec fn spec_index<I>(self, i: I) -> u32;
}

impl BitIndex for u32 {
    open spec fn spec_index<I>(self, i: I) -> u32 {
        u32_bits(self, i)
    }
}

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidMessage(s: S, message_id: UInt32) -> bool;

pub open spec fn IsValidCpli(s: S, domain_id: UInt32, cpli: UInt32) -> bool;

pub open spec fn DomainSupportsFastChannel(s: S, domain_id: UInt32) -> bool;

pub open spec fn MessageSupportsFastChannel(s: S, message_id: UInt32) -> bool;

pub open spec fn IsValidFastChannel(s: S, domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> bool;

pub open spec fn FastChannelHasDoorbell(s: S, domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> u32;

pub open spec fn DoorbellWidthEncoding(s: S, domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> u32;

pub open spec fn FastChannelAddress(s: S, domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> u32;

pub open spec fn FastChannelSize(s: S, domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> u32;

pub open spec fn DoorbellAddress(s: S, domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> u32;

} // verus!
