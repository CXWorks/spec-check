pub open spec fn reset__3_8_2_6_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!ResetDomainExists(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (Bits(old_s.flags, 31, 3) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidResetFlags(old_s.flags) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsSupportedResetState(old_s, domain_id, reset_state) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!AgentMayResetDomain(old_s, caller, domain_id) ==> ResultEqual(result, DENIED))
    && (ResetOperationFailed(old_s, domain_id) ==> ResultEqual(result, GENERIC_ERROR))
    && (ResultEqual(result, SUCCESS) ==> (
        (Bits(old_s.flags, 0, 0) == 1 && Bits(old_s.flags, 2, 2) == 0) ==> ResetDomainResetTo(old_s, domain_id, reset_state)
        && (Bits(old_s.flags, 0, 0) == 1 && Bits(old_s.flags, 2, 2) == 1) ==> ReturnedOnReceiptOfRequest(old_s, domain_id)
        && (Bits(old_s.flags, 0, 0) == 0 && Bits(old_s.flags, 1, 1) == 1) ==> ResetSignalAsserted(old_s, domain_id, reset_state)
        && (Bits(old_s.flags, 0, 0) == 0 && Bits(old_s.flags, 1, 1) == 0) ==> !ResetSignalAsserted(old_s, domain_id, reset_state)
    ))
}