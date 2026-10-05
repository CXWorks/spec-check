pub open spec fn power_state_set__3_3_2_6_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!PowerDomainExists(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidPowerState(old_s, domain_id, power_state) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsRequestSupported(old_s, flags, domain_id, power_state) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMaySetPowerState(old_s, calling_agent, domain_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (
        (IsSynchronousRequest(old_s, flags, domain_id) && !IsApplicationProcessorDomain(old_s, domain_id)) ==> PowerDomainInState(old_s, domain_id, power_state)
        && (Bits64(flags, 0, 0) == 1 && !IsApplicationProcessorDomain(old_s, domain_id)) ==> PowerStateChangeScheduled(old_s, domain_id, power_state)
        && IsApplicationProcessorDomain(old_s, domain_id) ==> ReturnsBeforeApPowerDown(old_s, domain_id)
        && IsApplicationProcessorDomain(old_s, domain_id) ==> TransitionBeginsOnWfi(old_s, domain_id, power_state)
        && forall|p: PowerDomainId| (IsParentDomain(old_s, p, domain_id) ==> PowerStateRequestedForParent(old_s, p, power_state))
    ))
}