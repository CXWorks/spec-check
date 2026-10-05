pub open spec fn power_state_set__3_3_2_6_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!PowerDomainExists(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidPowerState(old_s, domain_id, power_state) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsRequestSupported(old_s, domain_id, flags, power_state) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMaySetPowerDomainState(old_s, caller, domain_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (IsSyncOnlyDomain(old_s, domain_id) || (!IsApDomain(old_s, domain_id) && flags.async == 0) ==> PowerDomainStateMatches(old_s, domain_id, power_state)))
    && (ResultEqual(result, SUCCESS) ==> (!IsApDomain(old_s, domain_id) && flags.async == 1 ==> PowerStateChangeScheduled(old_s, domain_id, power_state)))
    && (ResultEqual(result, SUCCESS) ==> (IsApDomain(old_s, domain_id) ==> CommandReturnedBeforeApPowerDown(old_s, caller_ap)))
    && (ResultEqual(result, SUCCESS) ==> (IsApDomain(old_s, domain_id) ==> TransitionStartsOnWfiObserved(old_s, domain_id, power_state)))
}