pub open spec fn performance_qos_capability_subtypes__3_5_6_6_spec(domain_id: UInt32, capability_type: UInt32, status: Int32, capability_subtypes: UInt32, old_s: S, new_s: S) -> bool {
    ((!IsValidPerformanceDomain(old_s, domain_id)
        && !(((capability_type & 0xFFu32) != 0u32) && (((capability_type & 0xFFu32) & (((capability_type & 0xFFu32) - 1u32) as u32)) != 0u32)))
        ==> status == NOT_FOUND)
    && ((IsValidPerformanceDomain(old_s, domain_id)
        && !(((capability_type & 0xFFu32) != 0u32) && (((capability_type & 0xFFu32) & (((capability_type & 0xFFu32) - 1u32) as u32)) != 0u32))
        && !IsValidQosCapabilityType(old_s, domain_id, capability_type))
        ==> status == NOT_FOUND)
    && ((IsValidPerformanceDomain(old_s, domain_id)
        && (((capability_type & 0xFFu32) != 0u32) && (((capability_type & 0xFFu32) & (((capability_type & 0xFFu32) - 1u32) as u32)) != 0u32)))
        ==> status == INVALID_PARAMETERS)
    && ((!IsValidPerformanceDomain(old_s, domain_id)
        && (((capability_type & 0xFFu32) != 0u32) && (((capability_type & 0xFFu32) & (((capability_type & 0xFFu32) - 1u32) as u32)) != 0u32)))
        ==> (status == NOT_FOUND || status == INVALID_PARAMETERS))
    && ((IsValidPerformanceDomain(old_s, domain_id)
        && !(((capability_type & 0xFFu32) != 0u32) && (((capability_type & 0xFFu32) & (((capability_type & 0xFFu32) - 1u32) as u32)) != 0u32))
        && IsValidQosCapabilityType(old_s, domain_id, capability_type))
        ==> (status == SUCCESS
            && capability_subtypes == QosCapabilitySubtypesOf(old_s, domain_id, capability_type)
            && (capability_subtypes & 0xFF00FF00u32) == 0u32))
    && new_s == old_s
}
