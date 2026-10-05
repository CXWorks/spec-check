pub open spec fn performance_qos_capability_subtypes__3_5_6_6_spec(
    result: Int32,
    domain_id: UInt32,
    capability_type: UInt32,
    capability_subtypes: UInt32,
    old_s: S,
    new_s: S
) -> bool {
    (!IsValidPerfDomain(domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidQosCapabilityType(domain_id, capability_type) ==> ResultEqual(result, NOT_FOUND))
    && (PopCount(Bits(capability_type, 7, 0)) > 1 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> (
        Bits(capability_subtypes, 31, 24) == 0
        && Bits(capability_subtypes, 23, 16) == SupportedOemQosSubtypes(domain_id, capability_type)
        && Bits(capability_subtypes, 15, 8) == 0
        && Bits(capability_subtypes, 7, 0) == SupportedArchitectedQosSubtypes(domain_id, capability_type)
    ))
    && (old_s == new_s)
}