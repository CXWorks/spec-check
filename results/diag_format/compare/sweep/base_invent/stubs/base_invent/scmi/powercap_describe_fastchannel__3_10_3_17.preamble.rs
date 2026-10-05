use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const NOT_FOUND: int32 = -4;

pub struct PowercapDomain {
    pub fastchannel_supported: Map<uint32, bool>,
}

pub struct S {
    pub powercap_domain_count: uint32,
    pub powercap_message_count: uint32,
    pub powercap_cpli_count: uint32,
    pub powercap_domains: Map<uint32, PowercapDomain>,
    pub powercap_fastchannel_addr_low: uint32,
    pub powercap_fastchannel_addr_high: uint32,
    pub powercap_fastchannel_size: uint32,
    pub powercap_fastchannel_doorbell_addr_low: uint32,
    pub powercap_fastchannel_doorbell_addr_high: uint32,
    pub powercap_fastchannel_doorbell_set_mask_low: uint32,
    pub powercap_fastchannel_doorbell_set_mask_high: uint32,
    pub powercap_fastchannel_doorbell_preserve_mask_low: uint32,
    pub powercap_fastchannel_doorbell_preserve_mask_high: uint32,
}

pub open spec fn is_valid_domain_id(old_s: S, domain_id: uint32) -> bool;

pub open spec fn is_valid_message_id(old_s: S, message_id: uint32) -> bool;

pub open spec fn is_valid_cpli(old_s: S, cpli: uint32) -> bool;

pub open spec fn is_fastchannel_supported(old_s: S, domain_id: uint32, message_id: uint32) -> bool;

} // verus!
