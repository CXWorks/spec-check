pub open spec fn performance_qos_config_set__3_5_6_14_spec(status: i32, old_s: S, new_s: S, domain_id: u32, capability: u32, flags: u32, qos_value: u32) -> bool {
    let type_bitmap: u32 = (capability >> 16u32) & 0xFFu32;
    let subtype_bitmap: u32 = capability & 0xFFu32;
    let platform_reset: bool = (flags & 0x10u32) != 0u32;
    let sibling_reset: bool = (flags & 0x08u32) != 0u32;
    let domain_reset: bool = (flags & 0x04u32) != 0u32;
    let async_flag: bool = (flags & 0x02u32) != 0u32;
    let any_reset: bool = platform_reset || sibling_reset || domain_reset;
    let multiple_type_bits: bool = type_bitmap != 0u32 && (type_bitmap & ((type_bitmap - 1) as u32)) != 0u32;
    let multiple_subtype_bits: bool = subtype_bitmap != 0u32 && (subtype_bitmap & ((subtype_bitmap - 1) as u32)) != 0u32;
    let not_found_cond: bool =
        (!platform_reset && (!IsValidPerformanceDomain(old_s, domain_id) || !IsValidQosCapability(old_s, domain_id, capability)))
        || (platform_reset && !IsPlatformQosCapability(old_s, capability));
    let invalid_cond: bool =
        multiple_type_bits
        || multiple_subtype_bits
        || (flags >> 5u32) != 0u32
        || !IsQosConfigFlagsSupported(old_s, domain_id, capability, flags)
        || (!any_reset && !IsQosValueSupported(old_s, domain_id, capability, qos_value));
    let denied_cond: bool = !IsAgentPermittedQosConfig(old_s, domain_id, capability);
    let is_documented_error: bool =
        (not_found_cond && status == NOT_FOUND)
        || (invalid_cond && status == INVALID_PARAMETERS)
        || (denied_cond && status == DENIED);
    (not_found_cond ==> is_documented_error)
    && (invalid_cond ==> is_documented_error)
    && (denied_cond ==> is_documented_error)
    && (status != SUCCESS ==> new_s == old_s)
    && ((!not_found_cond && !invalid_cond && !denied_cond) ==> status == SUCCESS)
    && ((status == SUCCESS && !async_flag && platform_reset) ==>
        (forall|d: u32| IsValidPerformanceDomain(old_s, d) && IsValidQosCapability(old_s, d, capability) ==>
            QosValue(new_s, d, capability) == QosPlatformDefault(old_s, d, capability)))
    && ((status == SUCCESS && !async_flag && sibling_reset) ==>
        (forall|d: u32| (d == domain_id || IsSiblingDomain(old_s, domain_id, d)) && IsValidQosCapability(old_s, d, capability) ==>
            QosValue(new_s, d, capability) == QosPlatformDefault(old_s, d, capability)))
    && ((status == SUCCESS && !async_flag && domain_reset) ==>
        QosValue(new_s, domain_id, capability) == QosPlatformDefault(old_s, domain_id, capability))
    && ((status == SUCCESS && !async_flag && !any_reset) ==>
        QosValue(new_s, domain_id, capability) == qos_value)
}
