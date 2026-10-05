pub open spec fn affinity_info_spec(target_affinity: UInt64, lowest_affinity_level: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!AffinityInstanceIsPresent(old_s, target_affinity, lowest_affinity_level) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (lowest_affinity_level > 0 && !SupportsAffinityLevelAboveZero(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (AffinityInstanceIsDisabled(old_s, target_affinity, lowest_affinity_level) ==> ResultEqual(result, DISABLED))
  && (ResultEqual(result, ON) || ResultEqual(result, OFF) || ResultEqual(result, ON_PENDING) ==> true)
  && ((exists c: Core| InAffinityInstance(c, target_affinity, lowest_affinity_level) && (CoreEnabledByCpuOn(c) || IsColdBootPrimaryCore(c)) && !CoreHasCalledCpuOff(c)) ==> ResultEqual(result, ON))
  && ((forall c: Core| InAffinityInstance(c, target_affinity, lowest_affinity_level) ==> CpuOffProcessed(c)) ==> ResultEqual(result, OFF))
  && (((exists c: Core| InAffinityInstance(c, target_affinity, lowest_affinity_level) && CoreIsOnPending(c)) && (forall c: Core| InAffinityInstance(c, target_affinity, lowest_affinity_level) ==> (CoreIsOnPending(c) || CpuOffProcessed(c)))) ==> ResultEqual(result, ON_PENDING))
  && ((AffinityInstanceIsPresent(old_s, target_affinity, lowest_affinity_level) &&
       !(lowest_affinity_level > 0 && !SupportsAffinityLevelAboveZero(old_s)) &&
       !AffinityInstanceIsDisabled(old_s, target_affinity, lowest_affinity_level))
    ==> ResultEqual(result, ON) || ResultEqual(result, OFF) || ResultEqual(result, ON_PENDING))
}