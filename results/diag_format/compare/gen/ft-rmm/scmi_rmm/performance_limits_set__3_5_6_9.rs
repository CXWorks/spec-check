pub open spec fn performance_limits_set__3_5_6_9_spec(domain_id: UInt32, range_max: UInt32, range_min: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!PerformanceDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!LimitsWithinDescribedLevels(old_s, domain_id, range_max, range_min) ==> ResultEqual(status, OUT_OF_RANGE))
  && (range_max == 0 && range_min == 0 ==> ResultEqual(status, OUT_OF_RANGE))
  && (!AgentMayChangePerformanceLimits(old_s, domain_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> PerformanceDomain(new_s, domain_id).range_max == range_max)
  && (ResultEqual(status, SUCCESS) && range_max == 0 ==> PerformanceDomain(new_s, domain_id).range_max == PerformanceDomain(new_s, domain_id).range_max)
  && (ResultEqual(status, SUCCESS) && range_min != 0 ==> PerformanceDomain(new_s, domain_id).range_min == range_min)
  && (ResultEqual(status, SUCCESS) && range_min == 0 ==> PerformanceDomain(new_s, domain_id).range_min == PerformanceDomain(new_s, domain_id).range_min)
  && ((PerformanceDomainExists(old_s, domain_id) &&
       LimitsWithinDescribedLevels(old_s, domain_id, range_max, range_min) &&
       !(range_max == 0 && range_min == 0) &&
       AgentMayChangePerformanceLimits(old_s, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> PerformanceDomain(new_s, domain_id).range_max == PerformanceDomain(old_s, domain_id).range_max)
  && (result != SUCCESS
    ==> PerformanceDomain(new_s, domain_id).range_min == PerformanceDomain(old_s, domain_id).range_min)
}