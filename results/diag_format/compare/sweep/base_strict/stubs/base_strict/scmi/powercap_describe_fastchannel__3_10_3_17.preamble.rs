use vstd::prelude::*;

verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;
pub type RmiStatusCode = u64;

pub const RMI_SUCCESS: RmiStatusCode = 0;
pub const RMI_ERROR_NOT_FOUND: RmiStatusCode = 1;
pub const RMI_ERROR_NOT_SUPPORTED: RmiStatusCode = 2;

pub struct S {
    pub dummy: u64,
}

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn IsValidPowercapDomain(domain_id: UInt32) -> bool;

pub open spec fn IsValidPowercapMessage(message_id: UInt32) -> bool;

pub open spec fn IsValidCpli(domain_id: UInt32, cpli: UInt32) -> bool;

pub open spec fn DomainSupportsFastChannel(domain_id: UInt32) -> bool;

pub open spec fn MessageSupportsFastChannel(message_id: UInt32) -> bool;

pub open spec fn FastChannelHasDoorbell(domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> bool;

pub open spec fn DoorbellWidth(domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> UInt32;

pub open spec fn FastChannelRateLimitUs(domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> UInt32;

pub open spec fn FastChannelAddress(domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> UInt32;

pub open spec fn FastChannelSizeBytes(domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> UInt32;

pub open spec fn DoorbellAddress(domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> UInt32;

pub open spec fn DoorbellSetMask(domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> UInt32;

pub open spec fn DoorbellPreserveMask(domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> UInt32;

pub open spec fn DoorbellWriteClearsBitsNotInSetOrPreserveMask(domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> bool;

} // verus!
