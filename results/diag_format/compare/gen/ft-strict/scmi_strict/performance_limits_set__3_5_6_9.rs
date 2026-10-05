pub open spec fn performance_limits_set__3_5_6_9_spec(domain_id: UInt32, range_max: UInt32, range_min: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!PerfDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (range_max == 0 && range_min == 0 ==> ResultEqual(status, OUT_OF_RANGE))
  && (range_max != 0 && !IsWithinDescribedLevels(old_s, domain_id, range_max) ==> ResultEqual(status, OUT_OF_RANGE))
  && (range_min != 0 && !IsWithinDescribedLevels(old_s, domain_id, range_min) ==> ResultEqual(status, OUT_OF_RANGE))
  && (!CallerMayChangePerfLimits(old_s, domain_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) && range_max != 0 ==> PerfLimitMax(new_s, domain_id) == LimitFieldToLevel(new_s, domain_id, range_max))
  && (ResultEqual(status, SUCCESS) && range_max == 0 ==> PerfLimitMax(new_s, domain_id) == PrevPerfLimitMax(new_s, domain_id))
  && (ResultEqual(status, SUCCESS) && range_min != 0 ==> PerfLimitMin(new_s, domain_id) == LimitFieldToLevel(new_s, domain_id, range_min))
  && (ResultEqual(status, SUCCESS) && range_min == 0 ==> PerfLimitMin(new_s, domain_id) == PrevPerfLimitMin(new_s, domain_id))
  && (ResultEqual(status, SUCCESS) ==> PerfLevelEventuallyWithinLimits(new_s, domain_id))
  && ((PerfDomainExists(old_s, domain_id) &&
       !(range_max == 0 && range_min == 0) &&
       !(range_max != 0 && !IsWithinDescribedLevels(old_s, domain_id, range_max)) &&
       !(range_min != 0 && !IsWithinDescribedLevels(old_s, domain_id, range_min)) &&
       CallerMayChangePerfLimits(old_s, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> PerfLimitMax(new_s, domain_id) == PerfLimitMax(old_s, domain_id))
  && (result != SUCCESS
    ==> PerfLimitMin(new_s, domain_id) == PerfLimitMin(old_s, domain_id))
}