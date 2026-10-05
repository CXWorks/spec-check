pub open spec fn performance_qos_capability_subtypes__3_5_6_6_spec(
    result: Int32,
    old_s: S,
    new_s: S,
    domain_id: UInt32,
    capability_type: UInt32,
    capability_subtypes: UInt32,
) -> bool {
    (!IsValidPerfDomain(domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (BitCount(capability_type as int & 0xFF) > 1 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidQosCapabilityType(domain_id, capability_type) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> capability_subtypes == SupportedQosCapabilitySubtypes(domain_id, capability_type))
}