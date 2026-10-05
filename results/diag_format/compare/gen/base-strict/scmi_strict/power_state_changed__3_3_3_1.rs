pub open spec fn power_state_changed__3_3_3_1_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S, recipient: AgentId, domain_id: UInt32, agent_id: UInt32, power_state: UInt32) -> bool {
    (!AgentRegisteredForPowerStateNotification(old_s, recipient, domain_id) ==> !NotificationSentTo(old_s, recipient, domain_id))
    && PowerStateTransitionCompleted(old_s, new_s, domain_id, power_state)
    && PowerTransitionCausedBy(old_s, new_s, domain_id, agent_id)
}