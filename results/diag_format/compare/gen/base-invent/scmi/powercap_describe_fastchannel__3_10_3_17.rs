pub open spec fn powercap_describe_fastchannel__3_10_3_17_spec(
    result: int32,
    old_s: S,
    new_s: S,
    domain_id: uint32,
    message_id: uint32,
    cpli: uint32,
    attributes: uint32,
    rate_limit: uint32,
    chan_addr_low: uint32,
    chan_addr_high: uint32,
    chan_size: uint32,
    doorbell_addr_low: uint32,
    doorbell_addr_high: uint32,
    doorbell_set_mask_low: uint32,
    doorbell_set_mask_high: uint32,
    doorbell_preserve_mask_low: uint32,
    doorbell_preserve_mask_high: uint32,
) -> bool {
    // Failure conditions
    (!is_valid_domain_id(old_s, domain_id) ==> result == NOT_FOUND)
    && (!is_valid_message_id(old_s, message_id) ==> result == NOT_FOUND)
    && (!is_valid_cpli(old_s, cpli) ==> result == NOT_FOUND)
    && (!is_fastchannel_supported(old_s, domain_id, message_id) ==> result == NOT_SUPPORTED)
    // Success conditions
    && (is_valid_domain_id(old_s, domain_id)
        && is_valid_message_id(old_s, message_id)
        && is_valid_cpli(old_s, cpli)
        && is_fastchannel_supported(old_s, domain_id, message_id)
        ==> result == SUCCESS
        && attributes == 0
        && rate_limit == 0
        && chan_addr_low == new_s.powercap_fastchannel_addr_low
        && chan_addr_high == new_s.powercap_fastchannel_addr_high
        && chan_size == new_s.powercap_fastchannel_size
        && doorbell_addr_low == new_s.powercap_fastchannel_doorbell_addr_low
        && doorbell_addr_high == new_s.powercap_fastchannel_doorbell_addr_high
        && doorbell_set_mask_low == new_s.powercap_fastchannel_doorbell_set_mask_low
        && doorbell_set_mask_high == new_s.powercap_fastchannel_doorbell_set_mask_high
        && doorbell_preserve_mask_low == new_s.powercap_fastchannel_doorbell_preserve_mask_low
        && doorbell_preserve_mask_high == new_s.powercap_fastchannel_doorbell_preserve_mask_high)
}

fn is_valid_domain_id(old_s: S, domain_id: uint32) -> bool {
    domain_id < old_s.powercap_domain_count
}

fn is_valid_message_id(old_s: S, message_id: uint32) -> bool {
    message_id < old_s.powercap_message_count
}

fn is_valid_cpli(old_s: S, cpli: uint32) -> bool {
    cpli == 0 || cpli < old_s.powercap_cpli_count
}

fn is_fastchannel_supported(old_s: S, domain_id: uint32, message_id: uint32) -> bool {
    old_s.powercap_domains[domain_id].fastchannel_supported[message_id]
}