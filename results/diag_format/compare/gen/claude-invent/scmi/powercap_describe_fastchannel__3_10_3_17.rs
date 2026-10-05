pub open spec fn powercap_describe_fastchannel__3_10_3_17_spec(
    domain_id: u32,
    message_id: u32,
    cpli: u32,
    status: i32,
    attributes: u32,
    rate_limit: u32,
    chan_addr_low: u32,
    chan_addr_high: u32,
    chan_size: u32,
    doorbell_addr_low: u32,
    doorbell_addr_high: u32,
    doorbell_set_mask_low: u32,
    doorbell_set_mask_high: u32,
    doorbell_preserve_mask_low: u32,
    doorbell_preserve_mask_high: u32,
    old_s: S,
    new_s: S,
) -> bool {
    ((!IsValidPowercapDomain(old_s, domain_id)
        || !IsValidPowercapMessage(old_s, message_id)
        || !IsValidPowercapCpli(old_s, domain_id, message_id, cpli))
        ==> status == NOT_FOUND)
    && ((IsValidPowercapDomain(old_s, domain_id)
        && IsValidPowercapMessage(old_s, message_id)
        && IsValidPowercapCpli(old_s, domain_id, message_id, cpli)
        && (!PowercapDomainSupportsFastChannel(old_s, domain_id)
            || !PowercapMessageSupportsFastChannel(old_s, message_id)))
        ==> status == NOT_SUPPORTED)
    && ((IsValidPowercapDomain(old_s, domain_id)
        && IsValidPowercapMessage(old_s, message_id)
        && IsValidPowercapCpli(old_s, domain_id, message_id, cpli)
        && PowercapDomainSupportsFastChannel(old_s, domain_id)
        && PowercapMessageSupportsFastChannel(old_s, message_id))
        ==> (status == SUCCESS
            && (attributes >> 3u32) == 0u32
            && (rate_limit >> 20u32) == 0u32
            && ((chan_addr_high as int) * 0x1_0000_0000 + (chan_addr_low as int))
                == PowercapFastChannelAddress(old_s, domain_id, message_id, cpli)
            && (chan_size as int) >= PowercapFastChannelPayloadSize(message_id)
            && (((attributes & 1u32) == 1u32) ==>
                (((doorbell_addr_high as int) * 0x1_0000_0000 + (doorbell_addr_low as int))
                    == PowercapFastChannelDoorbellAddress(old_s, domain_id, message_id, cpli)))))
    && new_s == old_s
}
