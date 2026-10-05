pub open spec fn performance_qos_config_get__3_5_6_15_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!IsValidPerformanceDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidQosCapability(old_s, domain_id, capability) ==> ResultEqual(result, NOT_FOUND))
    && (QosCapabilityTypeBitCount(capability) > 1 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (QosCapabilitySubtypeBitCount(capability) > 1 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (result.is_Ok() ==> qos_value == QosConfiguration(old_s, domain_id, capability))
    && (result.is_Ok() ==> old_s == new_s)
}