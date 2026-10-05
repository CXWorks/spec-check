pub open spec fn power_state_changed__3_3_3_1_spec(agent_id: UInt32, domain_id: UInt32, power_state: UInt32, old_s: S, new_s: S) -> bool {
  (IsRegisteredForPowerStateNotification(old_s, recipient_agent, domain_id) ==> NotificationSentTo(new_s, recipient_agent))
  && (PowerStateTransitionCompleted(new_s, domain_id))
  && (PowerDomainAt(new_s, domain_id).state == power_state)
  && (PowerTransitionCausedBy(new_s, domain_id, agent_id))
  && ((!(IsRegisteredForPowerStateNotification(old_s, recipient_agent, domain_id)))
    ==> NotificationSentTo(new_s, recipient_agent))
  && (!(PowerStateTransitionCompleted(old_s, domain_id))
    ==> PowerStateTransitionCompleted(new_s, domain_id))
  && (!(PowerDomainAt(old_s, domain_id).state == power_state)
    ==> PowerDomainAt(new_s, domain_id).state == power_state)
  && (!(PowerTransitionCausedBy(old_s, domain_id, agent_id))
    ==> PowerTransitionCausedBy(new_s, domain_id, agent_id))
}