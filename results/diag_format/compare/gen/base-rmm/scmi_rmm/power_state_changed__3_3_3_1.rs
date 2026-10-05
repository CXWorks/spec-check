pub open spec fn power_state_changed__3_3_3_1_spec(result: RsiCommandReturnCode, old_s: S, new_s: S, recipient_agent: u32, domain_id: u32, power_state: u32, agent_id: u32) -> bool {
    (IsRegisteredForPowerStateNotification(old_s, recipient_agent, domain_id) ==> NotificationSentTo(new_s, recipient_agent))
    && (PowerStateTransitionCompleted(new_s, domain_id))
    && (PowerDomainAt(new_s, domain_id).state == power_state)
    && (PowerTransitionCausedBy(new_s, domain_id, agent_id))
    && (old_s == new_s)
}