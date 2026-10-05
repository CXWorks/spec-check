pub open spec fn psci_stat_count_spec(target_cpu: Mpidr, power_state: PowerState, result: UInt64, old_s: S, new_s: S) -> bool {
  (!StatFunctionsImplemented(old_s) ==> result == 0)
  && (!IsPresentNode(old_s, target_cpu) ==> result == 0)
  && (!NodeSupportsState(old_s, target_cpu, power_state) ==> result == 0)
  && (result == StatCount(new_s, target_cpu, HighestLevelLocalState(new_s, power_state)))
  && (IsOsInitiatedMode(old_s) ==> LastManLevelField(new_s, power_state) is disregarded when the local state is selected)
  && (Only local low-power states are tracked. Run states are not tracked)
  && (The statistics include low-power states entered by any PSCI call, including CPU_OFF, CPU_FREEZE and SYSTEM_SUSPEND)
  && (The result wraps when it overflows the resolution of the return register. For SMC32 PSCI_STAT_RESIDENCY, this can happen after just over 1 hour, 11 minutes and 34 seconds)
  && ((result == 0) ==> StatFunctionsImplemented(new_s))
  && ((result != 0) ==> IsPresentNode(new_s, target_cpu))
  && ((result != 0) ==> NodeSupportsState(new_s, target_cpu, power_state))
}