pub open spec fn power_state_set__3_3_2_6_spec(flags: UInt32, domain_id: UInt32, power_state: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!PowerDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidPowerState(old_s, domain_id, power_state) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsRequestSupported(old_s, domain_id, flags, power_state) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMaySetPowerDomainState(old_s, caller, domain_id) ==> ResultEqual(status, DENIED))
  && (result == SUCCESS ==> PowerDomainStateMatches(new_s, domain_id, power_state))
  && ((!IsSyncOnlyDomain(old_s, domain_id) && (flags.async == 0)) ==> PowerDomainStateMatches(new_s, domain_id, power_state))
  && ((!IsApDomain(old_s, domain_id) && flags.async == 1) ==> PowerStateChangeScheduled(new_s, domain_id, power_state))
  && (IsApDomain(old_s, domain_id) ==> CommandReturnedBeforeApPowerDown(new_s, caller_ap))
  && (IsApDomain(old_s, domain_id) ==> TransitionStartsOnWfiObserved(new_s, domain_id, power_state))
  && ((PowerDomainExists(old_s, domain_id) &&
       IsValidPowerState(old_s, domain_id, power_state) &&
       IsRequestSupported(old_s, domain_id, flags, power_state) &&
       AgentMaySetPowerDomainState(old_s, caller, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> PowerDomainStateMatches(new_s, domain_id, power_state))
  && (result != SUCCESS
    ==> PowerStateChangeScheduled(new_s, domain_id, power_state))
  && (result != SUCCESS
    ==> CommandReturnedBeforeApPowerDown(new_s, caller_ap))
  && (result != SUCCESS
    ==> TransitionStartsOnWfiObserved(new_s, domain_id, power_state))
}