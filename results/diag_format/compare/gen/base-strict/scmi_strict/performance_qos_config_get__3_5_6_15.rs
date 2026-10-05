pub open spec fn performance_qos_config_get__3_5_6_15_spec(result: Int32, qos_value: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidPerformanceDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidQosCapabilityOfDomain(old_s, domain_id, capability) ==> ResultEqual(result, NOT_FOUND))
    && (QosCapabilityTypeBitCount(capability) > 1 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (QosCapabilitySubtypeBitCount(capability) > 1 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> qos_value == CurrentQosConfig(old_s, domain_id, capability))
}