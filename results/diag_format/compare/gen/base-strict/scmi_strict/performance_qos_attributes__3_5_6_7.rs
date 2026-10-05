pub open spec fn performance_qos_attributes__3_5_6_7_spec(
    status: Int32,
    qos_attribute_1: UInt32,
    name: UInt8[16],
    domain_id: UInt32,
    capability: UInt32,
    old_s: S,
    new_s: S
) -> bool {
    (!IsValidPerformanceDomain(domain_id) ==> ResultEqual(status, NOT_FOUND))
    && (!IsValidQosCapability(domain_id, capability) ==> ResultEqual(status, NOT_FOUND))
    && (QosCapabilityTypeBitCount(capability) > 1 ==> ResultEqual(status, INVALID_PARAMETERS))
    && (QosCapabilitySubtypeBitCount(capability) > 1 ==> ResultEqual(status, INVALID_PARAMETERS))
    && (ResultEqual(status, SUCCESS) ==> (qos_attribute_1 == QosCapabilityFirstAttribute(domain_id, capability) && QosCapabilityNameEqual(name, domain_id, capability) && IsNullTerminatedAscii(name, 16)))
}