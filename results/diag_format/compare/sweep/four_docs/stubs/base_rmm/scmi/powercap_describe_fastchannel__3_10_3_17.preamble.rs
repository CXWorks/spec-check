use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub dummy: u32,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const NOT_FOUND: i32 = -3;

pub const chan_addr_high: u64 = 1;
pub const chan_addr_low: u64 = 2;
pub const doorbell_addr_high: u64 = 3;
pub const doorbell_addr_low: u64 = 4;
pub const chan_size: u32 = 5;

pub spec const attributes: Seq<u32> = Seq::empty();
pub spec const rate_limit: Seq<u32> = Seq::empty();

pub open spec fn ResultEqual(result: i32, code: i32) -> bool;
pub open spec fn IsValidPowercapDomain(domain_id: UInt32) -> bool;
pub open spec fn IsValidMessage(message_id: UInt32) -> bool;
pub open spec fn IsValidCpli(domain_id: UInt32, cpli: UInt32) -> bool;
pub open spec fn DomainSupportsFastChannel(domain_id: UInt32) -> bool;
pub open spec fn MessageSupportsFastChannel(message_id: UInt32) -> bool;
pub open spec fn IsValidFastChannel(domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> bool;
pub open spec fn FastChannelHasDoorbell(domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> u32;
pub open spec fn DoorbellWidthEncoding(domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> u32;
pub open spec fn FastChannelAddress(domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> u64;
pub open spec fn FastChannelSize(domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> u32;
pub open spec fn DoorbellAddress(domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> u64;

} // verus!
