pub open spec fn performance_level_get__3_5_6_12_spec(domain_id: UInt32, status: Int32, performance_level: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidPerformanceDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> performance_level == CurrentPerformanceLevelOrIndex(new_s, domain_id))
  && ((IsValidPerformanceDomain(old_s, domain_id))
    ==> ResultEqual(status, SUCCESS))
}