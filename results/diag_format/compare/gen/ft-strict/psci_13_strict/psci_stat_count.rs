pub open spec fn psci_stat_count_spec(fid: UInt64, target_cpu: UInt64, power_state: UInt64, result: UInt64, old_s: S, new_s: S) -> bool {
  (!StatFunctionsImplemented(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsNodePresent(old_s, target_cpu) ==> ResultEqual(result, 0))
  && (!NodeSupportsState(old_s, target_cpu, power_state) ==> ResultEqual(result, 0))
  && (IsStatCountFid(old_s, fid) ==> ResultEqual(result, StatCount(old_s, target_cpu, HighestLevelLocalState(old_s, power_state)) % ResultModulus(old_s, fid)))
  && (IsStatResidencyFid(old_s, fid) ==> ResultEqual(result, StatResidencyMicroseconds(old_s, target_cpu, HighestLevelLocalState(old_s, power_state)) % ResultModulus(old_s, fid)))
  && (IsOsInitiatedMode(old_s) ==> HighestLevelLocalState(old_s, power_state) == HighestLevelLocalState(old_s, IgnoreLastManField(old_s, power_state)))
  && (!IsRunState(old_s, HighestLevelLocalState(old_s, power_state)) ==> true)
  && (StatsIncludeAllEntryMethods(old_s, target_cpu, HighestLevelLocalState(old_s, power_state)) ==> true)
  && ((StatFunctionsImplemented(old_s) &&
       IsNodePresent(old_s, target_cpu) &&
       NodeSupportsState(old_s, target_cpu, power_state) &&
       !(IsStatCountFid(old_s, fid)) &&
       !(IsStatResidencyFid(old_s, fid)) &&
       !IsOsInitiatedMode(old_s) &&
       IsRunState(old_s, HighestLevelLocalState(old_s, power_state)) &&
       !StatsIncludeAllEntryMethods(old_s, target_cpu, HighestLevelLocalState(old_s, power_state)))
    ==> ResultEqual(result, 0))
}