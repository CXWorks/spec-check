pub open spec fn reset_issued__3_8_4_1_spec(agent_id: UInt32, domain_id: UInt32, reset_state: UInt32, old_s: S, new_s: S) -> bool {
  (ResetDomainHasBeenReset(new_s, domain_id, reset_state))
  && (AgentRegisteredForResetNotifications(new_s, recipient_agent, domain_id))
  && (ResetCausedByAgent(new_s, domain_id, agent_id))
  && ((!(ResetDomainHasBeenReset(new_s, domain_id, reset_state)) &&
       !(AgentRegisteredForResetNotifications(new_s, recipient_agent, domain_id)) &&
       !(ResetCausedByAgent(new_s, domain_id, agent_id)))
    ==> (true))
}