pub open spec fn performance_describe_levels__3_5_6_8_spec(domain_id: UInt32, skip_index: UInt32, status: Int32, num_levels: UInt32, perf_levels: [{UInt32; 5}], old_s: S, new_s: S) -> bool {
  (!IsValidPerfDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> Bits(num_levels, 15, 12) == 0)
  && (ResultEqual(status, SUCCESS) ==> PerfLevelsStartAfterSkipped(new_s, domain_id, skip_index, perf_levels))
  && (ResultEqual(status, SUCCESS) ==> Bits(num_levels, 31, 16) == NumRemainingPerfLevels(new_s, domain_id, skip_index, Bits(num_levels, 11, 0)))
  && (ResultEqual(status, SUCCESS) ==> (forall (i: UInt32), (i + 1 < Bits(num_levels, 11, 0)) ==> PerfLevelValue(perf_levels, i) < PerfLevelValue(perf_levels, i + 1))))
  && (ResultEqual(status, SUCCESS) ==> (forall (i: UInt32), (i < Bits(num_levels, 11, 0)) ==> Bits(PerfLevelAttributes(perf_levels, i), 31, 16) == 0))
  && (ResultEqual(status, SUCCESS) ==> (forall (i: UInt32), (i < Bits(num_levels, 11, 0)) ==> Bits(PerfLevelAttributes(perf_levels, i), 15, 0) == WorstCaseTransitionLatencyUs(new_s, domain_id, PerfLevelValue(perf_levels, i)))))
  && (ResultEqual(status, SUCCESS) ==> (forall (i: UInt32), (i < Bits(num_levels, 11, 0)) ==> (PerfLevelPowerCost(perf_levels, i) == 0 || IsLinearPowerScale(new_s, domain_id, PerfLevelPowerCost(perf_levels, i))))))
  && (ResultEqual(status, SUCCESS) ==> (forall (i: UInt32), (i < Bits(num_levels, 11, 0) && PerfLevelIndicativeFreq(perf_levels, i) != 0) ==> PerfLevelIndicativeFreq(perf_levels, i) == DomainClockFrequencyKhz(new_s, domain_id, PerfLevelValue(perf_levels, i))))))
  && (ResultEqual(status, SUCCESS) ==> (forall (i: UInt32), (i < Bits(num_levels, 11, 0) && LevelIndexingModeEnabled(new_s, domain_id)) ==> PerfLevelIndex(perf_levels, i) == LevelIndexOf(new_s, domain_id, PerfLevelValue(perf_levels, i))))))
  && ((IsValidPerfDomain(old_s, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result == SUCCESS
    ==> ResultEqual(status, SUCCESS))
}