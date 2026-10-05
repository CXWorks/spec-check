pub open spec fn psci_stat_residency_spec(target_cpu: Mpidr, power_state: PowerState, result: UInt, old_s: S, new_s: S) -> bool {
  (!StatFunctionsImplemented(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsPresentNode(old_s, target_cpu) ==> ResultEqual(result, 0))
  && (!NodeSupportsState(old_s, StatNode(old_s, target_cpu, power_state), StatLocalState(power_state)) ==> ResultEqual(result, 0))
  && (result == NOT_SUPPORTED ==> StatNode(new_s, target_cpu, power_state) == NodeAt(new_s, target_cpu, HighestPowerLevel(new_s, power_state)))
  && (result == NOT_SUPPORTED ==> IsOsInitiatedMode(new_s) ==> StatLocalState(new_s, power_state) ignores LastManLevelField(new_s, power_state))
  && (result == NOT_SUPPORTED ==> IsStatCount(new_s, result) ==> ResultEqual(result, StateUseCount(new_s, StatNode(new_s, target_cpu, power_state), StatLocalState(new_s, power_state)) mod 2^ResultBits(new_s, result)))
  && (result == NOT_SUPPORTED ==> IsStatResidency(new_s, result) ==> ResultEqual(result, StateResidencyUs(new_s, StatNode(new_s, target_cpu, power_state), StatLocalState(new_s, power_state)) mod 2^ResultBits(new_s, result)))
  && ((StatFunctionsImplemented(old_s) &&
       IsPresentNode(old_s, target_cpu) &&
       NodeSupportsState(old_s, StatNode(old_s, target_cpu, power_state), StatLocalState(power_state)))
    ==> StatNode(new_s, target_cpu, power_state) == NodeAt(new_s, target_cpu, HighestPowerLevel(new_s, power_state)))
  && ((StatFunctionsImplemented(old_s) &&
       IsPresentNode(old_s, target_cpu) &&
       NodeSupportsState(old_s, StatNode(old_s, target_cpu, power_state), StatLocalState(power_state)))
    ==> IsOsInitiatedMode(new_s) ==> StatLocalState(new_s, power_state) ignores LastManLevelField(new_s, power_state))
  && ((StatFunctionsImplemented(old_s) &&
       IsPresentNode(old_s, target_cpu) &&
       NodeSupportsState(old_s, StatNode(old_s, target_cpu, power_state), StatLocalState(power_state)))
    ==> IsStatCount(new_s, result) ==> ResultEqual(result, StateUseCount(new_s, StatNode(new_s, target_cpu, power_state), StatLocalState(new_s, power_state)) mod 2^ResultBits(new_s, result)))
  && ((StatFunctionsImplemented(old_s) &&
       IsPresentNode(old_s, target_cpu) &&
       NodeSupportsState(old_s, StatNode(old_s, target_cpu, power_state), StatLocalState(power_state)))
    ==> IsStatResidency(new_s, result) ==> ResultEqual(result, StateResidencyUs(new_s, StatNode(new_s, target_cpu, power_state), StatLocalState(new_s, power_state)) mod 2^ResultBits(new_s, result)))
  && (result != NOT_SUPPORTED
    ==> StatNode(new_s, target_cpu, power_state) == NodeAt(new_s, target_cpu, HighestPowerLevel(new_s, power_state)))
  && (result != NOT_SUPPORTED
    ==> IsOsInitiatedMode(new_s) ==> StatLocalState(new_s, power_state) ignores LastManLevelField(new_s, power_state))
  && (result != NOT_SUPPORTED
    ==> IsStatCount(new_s, result) ==> ResultEqual(result, StateUseCount(new_s, StatNode(new_s, target_cpu, power_state), StatLocalState(new_s, power_state)) mod 2^ResultBits(new_s, result)))
  && (result != NOT_SUPPORTED
    ==> IsStatResidency(new_s, result) ==> ResultEqual(result, StateResidencyUs(new_s, StatNode(new_s, target_cpu, power_state), StatLocalState(new_s, power_state)) mod 2^ResultBits(new_s, result)))
}