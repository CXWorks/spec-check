pub open spec fn performance_describe_levels__3_5_6_8_spec(result: Int32, num_levels: UInt32, perf_levels: Array<UInt32, 5>, old_s: S, new_s: S) -> bool {
    (!IsValidPerfDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (
        Bits(num_levels, 15, 12) == 0
        && PerfLevelsStartAfterSkipped(domain_id, skip_index, perf_levels)
        && Bits(num_levels, 31, 16) == NumRemainingPerfLevels(domain_id, skip_index, Bits(num_levels, 11, 0))
        && (forall|i: UInt32| (i + 1 < Bits(num_levels, 11, 0)) ==> PerfLevelValue(perf_levels, i) < PerfLevelValue(perf_levels, i + 1))
        && (forall|i: UInt32| (i < Bits(num_levels, 11, 0)) ==> Bits(PerfLevelAttributes(perf_levels, i), 31, 16) == 0)
        && (forall|i: UInt32| (i < Bits(num_levels, 11, 0)) ==> Bits(PerfLevelAttributes(perf_levels, i), 15, 0) == WorstCaseTransitionLatencyUs(domain_id, PerfLevelValue(perf_levels, i)))
        && (forall|i: UInt32| (i < Bits(num_levels, 11, 0)) ==> (PerfLevelPowerCost(perf_levels, i) == 0 || IsLinearPowerScale(domain_id, PerfLevelPowerCost(perf_levels, i))))
        && (forall|i: UInt32| (i < Bits(num_levels, 11, 0) && PerfLevelIndicativeFreq(perf_levels, i) != 0) ==> PerfLevelIndicativeFreq(perf_levels, i) == DomainClockFrequencyKhz(domain_id, PerfLevelValue(perf_levels, i)))
        && (forall|i: UInt32| (i < Bits(num_levels, 11, 0) && LevelIndexingModeEnabled(domain_id)) ==> PerfLevelIndex(perf_levels, i) == LevelIndexOf(domain_id, PerfLevelValue(perf_levels, i)))
    ))
}