pub open spec fn performance_level_changed__3_5_8_2_spec(agent_id: UInt32, domain_id: UInt32, performance_level: UInt32, old_s: S, new_s: S) -> bool {
  (AgentIsSubscribedToLevelChange(new_s, recipient, domain_id) ==> PerformanceLevelChanged(new_s, domain_id))
  && (PerformanceLevelChanged(old_s, domain_id) ==> PerformanceLevelOf(new_s, domain_id) == performance_level)
  && ((!(AgentIsSubscribedToLevelChange(old_s, recipient, domain_id)) &&
       !(PerformanceLevelChanged(old_s, domain_id)))
    ==> PerformanceLevelOf(new_s, domain_id) == PerformanceLevelOf(old_s, domain_id))
}