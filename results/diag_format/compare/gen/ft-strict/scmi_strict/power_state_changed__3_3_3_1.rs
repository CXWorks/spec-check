pub open spec fn power_state_changed__3_3_3_1_spec(agent_id: UInt32, domain_id: UInt32, power_state: UInt32, recipient: Recipient, old_s: S, new_s: S) -> bool {
  (!AgentRegisteredForPowerStateNotification(old_s, recipient, domain_id) ==> !NotificationSentTo(new_s, recipient, domain_id))
  && (PowerStateTransitionCompleted(new_s, domain_id, power_state))
  && (PowerTransitionCausedBy(new_s, domain_id, agent_id))
  && ((AgentRegisteredForPowerStateNotification(old_s, recipient, domain_id))
    ==> NotificationSentTo(new_s, recipient, domain_id))
}