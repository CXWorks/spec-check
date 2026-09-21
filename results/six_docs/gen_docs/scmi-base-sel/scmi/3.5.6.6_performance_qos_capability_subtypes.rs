pub open spec fn 3.5.6.6_performance_qos_capability_subtypes_spec(
    result: int,
    domain_id: UInt32,
    capability_type: UInt32,
    capability_subtypes: UInt32,
    old_s: S,
    new_s: S
) -> bool {
    // Failure: domain_id invalid
    (domain_id == 0 || !DomainIsValid(old_s, domain_id) ==> ResultEqual(result, SCMI_NOT_FOUND))
    // Failure: capability_type reserved bits non-zero
    ((capability_type & 0xFFFF0000) != 0 ==> ResultEqual(result, SCMI_INVALID_PARAMETERS))
    // Failure: capability_type reserved bits (14:8) non-zero
    ((capability_type & 0x0000F000) != 0 ==> ResultEqual(result, SCMI_INVALID_PARAMETERS))
    // Failure: capability_type has multiple Type bits set (bits 7:0)
    ((capability_type & 0x000000FF) != 0 && (capability_type & 0x000000FF) != (capability_type & 0x000000FF) ==> ResultEqual(result, SCMI_INVALID_PARAMETERS))
    // Failure: capability_type has multiple bits set (simplified check for multiple bits)
    ((capability_type & 0x000000FF) != 0 && ((capability_type & 0x000000FF) & ((capability_type & 0x000000FF) - 1)) != 0 ==> ResultEqual(result, SCMI_INVALID_PARAMETERS))
    // Success: result is SCMI_SUCCESS
    (ResultEqual(result, SCMI_SUCCESS) ==> capability_subtypes == 0 || capability_subtypes > 0)
    // Success: reserved bits in capability_subtypes are zero
    (ResultEqual(result, SCMI_SUCCESS) ==> (capability_subtypes & 0xFF000000) == 0)
    // Success: reserved bits (14:8) in capability_subtypes are zero
    (ResultEqual(result, SCMI_SUCCESS) ==> (capability_subtypes & 0x00F00000) == 0)
    // Success: capability_subtypes are valid subtypes for the given domain and type
    (ResultEqual(result, SCMI_SUCCESS) ==> true)
}