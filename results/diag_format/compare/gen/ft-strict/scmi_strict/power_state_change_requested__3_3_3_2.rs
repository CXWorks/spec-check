pub open spec fn power_state_change_requested__3_3_3_2_spec(agent_id: UInt32, domain_id: UInt32, power_state: UInt32, old_s: S, new_s: S) -> bool {
  (IsRegisteredForPowerStateChangeRequested(new_s, recipient_agent) &&
   agent_id != recipient_agent &&
   PlatformReceivedPowerStateChangeRequest(new_s, agent_id, domain_id, power_state))
}