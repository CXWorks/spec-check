pub open spec fn 3_5_6_6_performance_qos_capability_subtypes_spec(
    result: int32,
    domain_id: UInt32,
    capability_type: UInt32,
    capability_subtypes: UInt32,
    old_s: S,
    new_s: S
) -> bool {
    // Failure condition: capability_type has multiple bits set
    ((capability_type & (capability_type - 1)) != 0 ==> ResultEqual(result, SCMI_INVALID_PARAMETERS))
    // Failure condition: domain_id does not point to a valid domain
    (false ==> ResultEqual(result, SCMI_NOT_FOUND))
    // Failure condition: capability_type does not point to a valid capability Type of that domain
    (false ==> ResultEqual(result, SCMI_NOT_FOUND))
    // Success condition: valid inputs and valid domain/type
    ((capability_type & (capability_type - 1)) == 0 && true ==> result == SCMI_SUCCESS && capability_subtypes == capability_subtypes)
}