use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u32,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const NOT_FOUND: i32 = -3;

pub open spec fn IsValidPowercapDomain(s: S, domain_id: u32) -> bool;
pub open spec fn IsValidPowercapMessage(s: S, message_id: u32) -> bool;
pub open spec fn IsValidPowercapCpli(s: S, domain_id: u32, message_id: u32, cpli: u32) -> bool;
pub open spec fn PowercapDomainSupportsFastChannel(s: S, domain_id: u32) -> bool;
pub open spec fn PowercapMessageSupportsFastChannel(s: S, message_id: u32) -> bool;
pub open spec fn PowercapFastChannelAddress(s: S, domain_id: u32, message_id: u32, cpli: u32) -> int;
pub open spec fn PowercapFastChannelPayloadSize(message_id: u32) -> int;
pub open spec fn PowercapFastChannelDoorbellAddress(s: S, domain_id: u32, message_id: u32, cpli: u32) -> int;

} // verus!
