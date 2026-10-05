pub open spec fn reset__3_8_2_6_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!ResetDomainExists(old_s, domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidResetFlags(old_s, flags(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsSupportedResetState(old_s, domain_id(old_s), reset_state(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!AgentMayResetDomain(old_s, caller(old_s), domain_id(old_s)) ==> ResultEqual(result, DENIED))
    && (ResetOperationFailed(old_s, domain_id(old_s)) ==> ResultEqual(result, GENERIC_ERROR))
    && (ResultEqual(result, SUCCESS) ==> (
        (flags(old_s)[0] == 1 ==> ResetDomainAt(new_s, domain_id(old_s)).reset_autonomous)
        && ((flags(old_s)[0] == 0 && flags(old_s)[1] == 1) ==> ResetDomainAt(new_s, domain_id(old_s)).reset_signal == ASSERTED)
        && ((flags(old_s)[0] == 0 && flags(old_s)[1] == 0) ==> ResetDomainAt(new_s, domain_id(old_s)).reset_signal == DEASSERTED)
        && (flags(old_s)[0] == 1 && flags(old_s)[2] == 1 ==> true)
    ))
    && (ResultEqual(result, SUCCESS) ==> (
        ResetDomainAt(new_s, domain_id(old_s)).state == reset_state(old_s)
    ))
    && (ResultEqual(result, SUCCESS) ==> (
        ResetDomainAt(new_s, domain_id(old_s)).reset_signal == (
            if flags(old_s)[0] == 1 then old_s.ResetDomainAt(domain_id(old_s)).reset_signal
            else if flags(old_s)[1] == 1 then ASSERTED
            else DEASSERTED
        )
    ))
}