pub open spec fn psci_stat_count_spec(result: UInt64, old_s: S, new_s: S) -> bool {
    (!StatFunctionsImplemented(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsNodePresent(old_s, target_cpu) ==> ResultEqual(result, 0))
    && (!NodeSupportsState(old_s, target_cpu, power_state) ==> ResultEqual(result, 0))
    && (IsStatCountFid(fid) ==> ResultEqual(result, StatCount(old_s, target_cpu, HighestLevelLocalState(old_s, power_state)) % ResultModulus(fid)))
    && (IsStatResidencyFid(fid) ==> ResultEqual(result, StatResidencyMicroseconds(old_s, target_cpu, HighestLevelLocalState(old_s, power_state)) % ResultModulus(fid)))
    && (IsOsInitiatedMode(old_s) ==> HighestLevelLocalState(old_s, power_state) == HighestLevelLocalState(old_s, IgnoreLastManField(power_state)))
    && (!IsRunState(old_s, HighestLevelLocalState(old_s, power_state)))
    && (StatsIncludeAllEntryMethods(old_s, target_cpu, HighestLevelLocalState(old_s, power_state)))
    && (old_s == new_s)
}