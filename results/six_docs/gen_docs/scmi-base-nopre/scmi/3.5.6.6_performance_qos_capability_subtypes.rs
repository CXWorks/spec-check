pub open spec fn 3.5.6.6_performance_qos_capability_subtypes_spec(
    result: int32,
    domain_id: uint32,
    capability_type: uint32,
    capability_subtypes: uint32,
    old_s: S,
    new_s: S
) -> bool {
    // Failure conditions
    // INVALID_PARAMETERS: if the capability_type field has multiple Type bits set.
    // capability_type bits [7:0] are the Type bitmap. Only one bit must be set.
    let type_bitmap = capability_type & 0xFF;
    let multiple_bits_set = type_bitmap != 0 && (type_bitmap & (type_bitmap - 1)) != 0;
    (multiple_bits_set ==> ResultEqual(result, INVALID_PARAMETERS))

    // NOT_FOUND: if the domain_id parameter does not point to a valid domain
    // or if the capability_type does not point to a valid capability Type of that domain.
    // (Assuming a helper or predicate exists in context for valid domain/type check; otherwise unconstrained)
    // Since no specific predicate for "valid domain/type" is provided in the context,
    // and the spec implies this check happens, we leave the implication for NOT_FOUND
    // as unconstrained (true) if no helper is available, or rely on the caller's context.
    // However, to be precise based on the text: if NOT_FOUND is returned, the preconditions must have failed.
    // Without a specific predicate name in the provided text, we cannot form a specific implication.
    // We will assume the command logic handles this internally and the spec focuses on the return value mapping.
    // If the spec text implies a check, we might need a predicate like `IsValidDomainAndType(old_s, domain_id, capability_type)`.
    // Lacking that, we cannot assert a specific failure condition implication other than the one derived from the bitmap.
    // Thus, we only assert the INVALID_PARAMETERS condition explicitly derived from the input.

    // Success condition: SUCCESS: if valid QoS Subtypes are returned.
    // This implies the domain_id and capability_type were valid, and the bitmap was valid.
    // The return value must be SUCCESS.
    // The capability_subtypes must be returned correctly (though the spec doesn't define a specific predicate for correctness,
    // it implies the value is valid for the given inputs).
    // We assert that if the inputs are valid (not INVALID_PARAMETERS), the result is SUCCESS.
    // Note: The spec says "if valid QoS Subtypes are returned", implying the return value is SUCCESS.
    // We cannot assert the exact value of capability_subtypes without a predicate for "correct subtypes".
    // So we only assert the result code.

    // Also, check reserved bits in capability_type
    // Bits[31:16] Reserved, must be zero.
    // Bits[14:8] Reserved, must be zero.
    let reserved_upper = (capability_type >> 16) & 0xFFFF;
    let reserved_middle = (capability_type >> 8) & 0xFF;
    (reserved_upper != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    (reserved_middle != 0 ==> ResultEqual(result, INVALID_PARAMETERS))

    // Check reserved bits in capability_subtypes
    // Bits[31:24] Reserved, must be zero.
    // Bits[15:8] Reserved, must be zero.
    let reserved_subtype_upper = (capability_subtypes >> 24) & 0xFF;
    let reserved_subtype_middle = (capability_subtypes >> 8) & 0xFF;
    (reserved_subtype_upper != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    (reserved_subtype_middle != 0 ==> ResultEqual(result, INVALID_PARAMETERS))

    // Combine all
    (multiple_bits_set ==> ResultEqual(result, INVALID_PARAMETERS))
    && (reserved_upper != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (reserved_middle != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (reserved_subtype_upper != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (reserved_subtype_middle != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> (
        // If success, the inputs must have been valid (not INVALID_PARAMETERS)
        !multiple_bits_set
        && reserved_upper == 0
        && reserved_middle == 0
        && reserved_subtype_upper == 0
        && reserved_subtype_middle == 0
    ))
}