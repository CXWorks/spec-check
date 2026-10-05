pub open spec fn power_state_set__3_3_2_6_spec(flags: UInt32, domain_id: UInt32, power_state: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!PowerDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidPowerState(old_s, domain_id, power_state) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsRequestSupported(old_s, flags, domain_id, power_state) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMaySetPowerState(old_s, calling_agent, domain_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS))
  && ((IsSynchronousRequest(old_s, flags, domain_id) && !IsApplicationProcessorDomain(old_s, domain_id)) ==> PowerDomainInState(new_s, domain_id, power_state))
  && ((Bits(flags, 0, 0) == 1 && !IsApplicationProcessorDomain(old_s, domain_id)) ==> PowerStateChangeScheduled(new_s, domain_id, power_state))
  && (IsApplicationProcessorDomain(old_s, domain_id) ==> ReturnsBeforeApPowerDown(new_s, domain_id))
  && (IsApplicationProcessorDomain(old_s, domain_id) ==> TransitionBeginsOnWfi(new_s, domain_id, power_state))
  && (forall p: PowerDomainId| IsParentDomain(old_s, p, domain_id) ==> PowerStateRequestedForParent(new_s, p, power_state))
  && ((!(PowerDomainExists(old_s, domain_id)) &&
       IsValidPowerState(old_s, domain_id, power_state) &&
       IsRequestSupported(old_s, flags, domain_id, power_state) &&
       AgentMaySetPowerState(old_s, calling_agent, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> PowerDomainInState(new_s, domain_id, power_state))
  && (result != SUCCESS
    ==> PowerStateChangeScheduled(new_s, domain_id, power_state))
  && (result != SUCCESS
    ==> ReturnsBeforeApPowerDown(new_s, domain_id))
  && (result != SUCCESS
    ==> TransitionBeginsOnWfi(new_s, domain_id, power_state))
  && (result != SUCCESS
    ==> PowerStateRequestedForParent(new_s, p, power_state))
}