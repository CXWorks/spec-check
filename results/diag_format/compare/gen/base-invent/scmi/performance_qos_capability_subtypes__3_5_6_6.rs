pub open spec fn performance_qos_capability_subtypes__3_5_6_6_spec(
    result: int32,
    domain_id: uint32,
    capability_type: uint32,
    capability_subtypes: uint32,
    old_s: S,
    new_s: S
) -> bool {
    // Failure: domain_id does not point to a valid domain
    (!DomainIsValid(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    // Failure: capability_type does not point to a valid capability Type of that domain
    (!CapabilityTypeIsValid(old_s, domain_id, capability_type) ==> ResultEqual(result, NOT_FOUND))
    // Failure: capability_type field has multiple Type bits set
    (CountSetBits(capability_type) > 1 ==> ResultEqual(result, INVALID_PARAMETERS))
    // Failure: capability_type reserved bits are non-zero
    ((capability_type & 0xFFFF0000) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    // Failure: capability_type reserved bits (14:8) are non-zero
    ((capability_type & 0x0000FF00) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    // Failure: capability_subtypes reserved bits are non-zero
    ((capability_subtypes & 0xFF000000) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    // Failure: capability_subtypes reserved bits (15:8) are non-zero
    ((capability_subtypes & 0x00FF0000) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    // Success: valid domain, valid capability type, single bit set in capability_type, reserved bits zero
    (DomainIsValid(old_s, domain_id)
     && CapabilityTypeIsValid(old_s, domain_id, capability_type)
     && CountSetBits(capability_type) == 1
     && (capability_type & 0xFFFF0000) == 0
     && (capability_type & 0x0000FF00) == 0
     && (capability_subtypes & 0xFF000000) == 0
     && (capability_subtypes & 0x00FF0000) == 0
     ==> result == SUCCESS)
}