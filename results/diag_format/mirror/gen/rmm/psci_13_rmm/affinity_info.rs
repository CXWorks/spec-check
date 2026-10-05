pub open spec fn affinity_info_spec(lowest_affinity_level: UInt64, target_affinity: UInt64, result: Result<AffinityState, PsciStatusCode>, old_s: S, new_s: S) -> bool {
  (!IsAffinityInstancePresent(old_s, lowest_affinity_level, target_affinity) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (PsciVersion(old_s) >= 1.0 && lowest_affinity_level > 0 && !SupportsAffinityLevelAboveZero(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsAffinityInstanceDisabled(old_s, lowest_affinity_level, target_affinity) ==> ResultEqual(result, DISABLED))
  && (result == ON ==> (Exists core in AffinityInstance(old_s, lowest_affinity_level, target_affinity) such that (CoreEnabledByCpuOn(old_s, core) || IsColdBootPrimaryCore(old_s, core)) && !CoreCalledCpuOff(old_s, core))))
  && (result == OFF ==> (For all cores in AffinityInstance(old_s, lowest_affinity_level, target_affinity), CoreCalledCpuOff(old_s, core) && CpuOffProcessed(old_s, core))))
  && (result == ON_PENDING ==> (Exists core in AffinityInstance(old_s, lowest_affinity_level, target_affinity) such that CoreState(old_s, core) == ON_PENDING, and all other cores in the instance are OFF))
  && ((IsAffinityInstancePresent(old_s, lowest_affinity_level, target_affinity) &&
       !(PsciVersion(old_s) >= 1.0 && lowest_affinity_level > 0 && !SupportsAffinityLevelAboveZero(old_s)) &&
       !IsAffinityInstanceDisabled(old_s, lowest_affinity_level, target_affinity))
    ==> result != INVALID_PARAMETERS)
  && (result != ON && result != OFF && result != ON_PENDING && result != DISABLED && result != INVALID_PARAMETERS
    ==> true)
}