pub open spec fn performance_qos_config_get__3_5_6_15_spec(domain_id: u32, capability: u32, status: i32, qos_value: u32, old_s: S, new_s: S) -> bool {
    (!IsValidPerformanceDomain(old_s, domain_id) ==> status == NOT_FOUND)
    && ((IsValidPerformanceDomain(old_s, domain_id)
         && (QosCapabilityTypeBitCount(capability) != 1 || QosCapabilitySubtypeBitCount(capability) > 1))
        ==> status == INVALID_PARAMETERS)
    && ((IsValidPerformanceDomain(old_s, domain_id)
         && QosCapabilityTypeBitCount(capability) == 1
         && QosCapabilitySubtypeBitCount(capability) <= 1
         && !IsSupportedQosCapability(old_s, domain_id, capability))
        ==> status == NOT_FOUND)
    && ((status == SUCCESS)
        ==> (IsValidPerformanceDomain(old_s, domain_id)
             && QosCapabilityTypeBitCount(capability) == 1
             && QosCapabilitySubtypeBitCount(capability) <= 1
             && IsSupportedQosCapability(old_s, domain_id, capability)
             && qos_value == PerformanceQosConfig(old_s, domain_id, capability)))
    && (new_s == old_s)
}
