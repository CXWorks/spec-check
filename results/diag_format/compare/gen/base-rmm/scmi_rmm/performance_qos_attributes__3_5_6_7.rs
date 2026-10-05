pub open spec fn performance_qos_attributes__3_5_6_7_spec(result: int32, qos_attribute_1: uint32, name: [uint8; 16], old_s: S, new_s: S) -> bool {
    (!IsValidPerformanceDomain(domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidQosCapabilityOfDomain(domain_id, capability) ==> ResultEqual(result, NOT_FOUND))
    && (CountQosCapabilityTypeBits(capability) > 1 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (CountQosCapabilitySubtypeBits(capability) > 1 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> (qos_attribute_1 == QosCapabilityFirstAttribute(domain_id, capability) && name == QosCapabilityName(domain_id, capability)))
    && (old_s == new_s)
}