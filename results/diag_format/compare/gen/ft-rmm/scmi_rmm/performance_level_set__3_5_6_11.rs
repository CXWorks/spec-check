pub open spec fn performance_level_set__3_5_6_11_spec(domain_id: UInt32, performance_level: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidPerformanceDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsInAllowedPerformanceRange(old_s, domain_id, performance_level) ==> ResultEqual(status, OUT_OF_RANGE))
  && (!AgentMaySetPerformanceLevel(old_s, caller, domain_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> PlatformHasAcceptedAndScheduled(new_s, domain_id, performance_level))
  && ((IsValidPerformanceDomain(old_s, domain_id) &&
       IsInAllowedPerformanceRange(old_s, domain_id, performance_level) &&
       AgentMaySetPerformanceLevel(old_s, caller, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> PerformanceDomain(new_s, domain_id).performance_level == PerformanceDomain(old_s, domain_id).performance_level)
}