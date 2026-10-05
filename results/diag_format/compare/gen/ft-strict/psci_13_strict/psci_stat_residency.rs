pub open spec fn psci_stat_residency_spec(target_cpu: UInt64, power_state: UInt64, result: UInt64, old_s: S, new_s: S) -> bool {
  (!StatFunctionsImplemented(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!NodePresent(old_s, StatNode(target_cpu, power_state)) ==> ResultEqual(result, 0))
  && (!NodeSupportsLocalState(old_s, StatNode(target_cpu, power_state), StatLocalState(power_state)) ==> ResultEqual(result, 0))
  && (IsStatCount(old_s, result) ==> ResultEqual(result, StatEntryCount(old_s, StatNode(target_cpu, power_state), StatLocalState(power_state)) % ResultModulus(result)))
  && (IsStatResidency(old_s, result) ==> ResultEqual(result, StatResidencyMicroseconds(old_s, StatNode(target_cpu, power_state), StatLocalState(power_state)) % ResultModulus(result)))
  && (IsOsInitiatedMode(old_s) ==> StatLocalState(power_state) == StatLocalState(DisregardLastInLevelField(power_state)))
  && (IsLocalLowPowerState(old_s, StatLocalState(power_state)) ==> true)
  && (StatsCountAllEntryMethods(old_s, StatNode(target_cpu, power_state), StatLocalState(power_state)) ==> true)
  && (StatsZeroedAtColdBoot(old_s) && StatsZeroedBySystemResetAndShutdown(old_s) ==> true)
  && ((StatFunctionsImplemented(old_s) &&
       NodePresent(old_s, StatNode(target_cpu, power_state)) &&
       NodeSupportsLocalState(old_s, StatNode(target_cpu, power_state), StatLocalState(power_state)) &&
       !(IsStatCount(old_s, result)) &&
       !(IsStatResidency(old_s, result)) &&
       !(IsOsInitiatedMode(old_s)) &&
       !(IsLocalLowPowerState(old_s, StatLocalState(power_state))) &&
       !(StatsCountAllEntryMethods(old_s, StatNode(target_cpu, power_state), StatLocalState(power_state))) &&
       !(StatsZeroedAtColdBoot(old_s) && StatsZeroedBySystemResetAndShutdown(old_s)))
    ==> ResultEqual(result, 0))
}