pub open spec fn performance_level_get__3_5_6_12_spec(domain_id: uint32, status: int32, performance_level: uint32, old_s: S, new_s: S) -> bool {
  (!IsValidPerformanceDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> performance_level == CurrentPerformanceLevel(new_s, domain_id))
  && ((IsValidPerformanceDomain(old_s, domain_id))
    ==> ResultEqual(status, SUCCESS))
}