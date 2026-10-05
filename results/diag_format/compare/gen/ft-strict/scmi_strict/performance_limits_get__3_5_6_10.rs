pub open spec fn performance_limits_get__3_5_6_10_spec(domain_id: UInt32, status: Int32, range_max: UInt32, range_min: UInt32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!IsValidPerformanceDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
  && (result.is_Ok() ==> ResultEqual(status, SUCCESS))
  && (result.is_Ok() ==> range_max == CurrentPerformanceLimitMax(new_s, domain_id))
  && (result.is_Ok() ==> range_min == CurrentPerformanceLimitMin(new_s, domain_id))
  && ((IsValidPerformanceDomain(old_s, domain_id))
    ==> result.is_Ok())
}