pub open spec fn performance_limits_get__3_5_6_10_spec(domain_id: uint32, status: int32, range_max: uint32, range_min: uint32, old_s: S, new_s: S) -> bool {
  (!IsValidPerformanceDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (result == SUCCESS ==> ResultEqual(status, SUCCESS))
  && (result == SUCCESS ==> range_max == PerformanceDomain(new_s, domain_id).limits.max)
  && (result == SUCCESS ==> range_min == PerformanceDomain(new_s, domain_id).limits.min)
  && ((IsValidPerformanceDomain(old_s, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> PerformanceDomain(new_s, domain_id).limits.max == PerformanceDomain(old_s, domain_id).limits.max)
  && (result != SUCCESS
    ==> PerformanceDomain(new_s, domain_id).limits.min == PerformanceDomain(old_s, domain_id).limits.min)
}