use vstd::prelude::*;
verus! {

pub type UInt32 = u64;
pub type UInt64 = u64;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1int) as Int32;
pub spec const NOT_FOUND: Int32 = (-4int) as Int32;
pub spec const result: Int32 = 7;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(x: UInt64, hi: int, lo: int) -> UInt64;

pub open spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidPowercapMessage(s: S, message_id: UInt32) -> bool;

pub open spec fn IsValidCpli(s: S, domain_id: UInt32, cpli: UInt32) -> bool;

pub open spec fn DomainSupportsFastChannel(s: S, domain_id: UInt32) -> bool;

pub open spec fn MessageSupportsFastChannel(s: S, message_id: UInt32) -> bool;

pub open spec fn FastChannelHasDoorbell(s: S, domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> bool;

pub open spec fn DoorbellWidth(s: S, domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> int;

pub open spec fn FastChannelRateLimitUs(s: S, domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> UInt64;

pub open spec fn FastChannelAddress(s: S, domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> UInt64;

pub open spec fn FastChannelSizeBytes(s: S, domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> UInt64;

pub open spec fn DoorbellAddress(s: S, domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> UInt64;

pub open spec fn DoorbellSetMask(s: S, domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> UInt64;

pub open spec fn DoorbellPreserveMask(s: S, domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> UInt64;

pub open spec fn DoorbellWriteClearsBitsNotInSetOrPreserveMask(s: S, domain_id: UInt32, message_id: UInt32, cpli: UInt32) -> bool;

} // verus!
