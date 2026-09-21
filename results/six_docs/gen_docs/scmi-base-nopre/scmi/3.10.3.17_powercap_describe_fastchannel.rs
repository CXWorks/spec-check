pub open spec fn 3.10.3.17_powercap_describe_fastchannel_spec(
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
    (result == 0 ==> (
        (old_s.powercap_domains[domain_id as int].is_some() &&
         old_s.powercap_domains[domain_id as int].unwrap().messages[message_id as int].is_some() &&
         old_s.powercap_domains[domain_id as int].unwrap().messages[message_id as int].unwrap().cplis[cpli as int].is_some())
        &&
        (new_s.powercap_domains[domain_id as int] == old_s.powercap_domains[domain_id as int])
        &&
        (new_s.powercap_domains[domain_id as int].unwrap().messages[message_id as int] == old_s.powercap_domains[domain_id as int].unwrap().messages[message_id as int])
        &&
        (new_s.powercap_domains[domain_id as int].unwrap().messages[message_id as int].unwrap().cplis[cpli as int] == old_s.powercap_domains[domain_id as int].unwrap().messages[message_id as int].unwrap().cplis[cpli as int])
        &&
        (attributes == 0 || (attributes & 0xFFFFFFFE) == 0)
        &&
        (rate_limit & 0xFFFFF000) == 0
        &&
        (doorbell_addr_low == 0 || doorbell_addr_high == 0 || doorbell_set_mask_low == 0 || doorbell_set_mask_high == 0 || doorbell_preserve_mask_low == 0 || doorbell_preserve_mask_high == 0)
    ))
    &&
    (result == 1 ==> (
        (old_s.powercap_domains[domain_id as int].is_none() ||
         old_s.powercap_domains[domain_id as int].unwrap().messages[message_id as int].is_none() ||
         old_s.powercap_domains[domain_id as int].unwrap().messages[message_id as int].unwrap().cplis[cpli as int].is_none())
    ))
    &&
    (result == 2 ==> (
        (old_s.powercap_domains[domain_id as int].is_some() &&
         old_s.powercap_domains[domain_id as int].unwrap().messages[message_id as int].is_some() &&
         old_s.powercap_domains[domain_id as int].unwrap().messages[message_id as int].unwrap().cplis[cpli as int].is_some() &&
         !old_s.powercap_domains[domain_id as int].unwrap().messages[message_id as int].unwrap().fast_channel_supported)
    ))
    &&
    (result != 0 && result != 1 && result != 2 ==> true)
}