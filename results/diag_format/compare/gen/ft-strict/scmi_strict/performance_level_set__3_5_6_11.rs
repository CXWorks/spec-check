pub open spec fn performance_level_set__3_5_6_11_spec(domain_id: UInt32, performance_level: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidPerformanceDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsWithinAllowedPerformanceRange(old_s, domain_id, performance_level) ==> ResultEqual(status, OUT_OF_RANGE))
  && (!AgentPermittedToSetPerformanceLevel(old_s, calling_agent, domain_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> PerformanceLevelRequestScheduled(new_s, domain_id, performance_level))
  && ((!LevelIndexingModeInUse(old_s, domain_id) && performance_level == 0) ==> PlatformPolicyDeterminesPerformanceLevel(new_s, domain_id))
  && ((IsValidPerformanceDomain(old_s, domain_id) &&
       IsWithinAllowedPerformanceRange(old_s, domain_id, performance_level) &&
       AgentPermittedToSetPerformanceLevel(old_s, calling_agent, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, NOT_FOUND)
    ==> !PerformanceLevelRequestScheduled(new_s, domain_id, performance_level))
  && (ResultEqual(status, OUT_OF_RANGE)
    ==> !PerformanceLevelRequestScheduled(new_s, domain_id, performance_level))
  && (ResultEqual(status, DENIED)
    ==> !PerformanceLevelRequestScheduled(new_s, domain_id, performance_level))
  && (result != SUCCESS
    ==> !PerformanceLevelRequestScheduled(new_s, domain_id, performance_level))
}