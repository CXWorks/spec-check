pub open spec fn psci_stat_residency_spec(result: UInt64, old_s: S, new_s: S) -> bool {
    (!StatFunctionsImplemented() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!NodePresent(StatNode(old_s, target_cpu, power_state)) ==> ResultEqual(result, 0))
    && (!NodeSupportsLocalState(StatNode(old_s, target_cpu, power_state), StatLocalState(power_state)) ==> ResultEqual(result, 0))
    && (IsStatCount(fid) ==> ResultEqual(result, StatEntryCount(StatNode(old_s, target_cpu, power_state), StatLocalState(power_state)) % ResultModulus(fid)))
    && (IsStatResidency(fid) ==> ResultEqual(result, StatResidencyMicroseconds(StatNode(old_s, target_cpu, power_state), StatLocalState(power_state)) % ResultModulus(fid)))
    && (IsOsInitiatedMode() ==> StatLocalState(power_state) == StatLocalState(DisregardLastInLevelField(power_state)))
    && IsLocalLowPowerState(StatLocalState(power_state))
    && StatsCountAllEntryMethods(StatNode(old_s, target_cpu, power_state), StatLocalState(power_state))
    && StatsZeroedAtColdBoot()
    && StatsZeroedBySystemResetAndShutdown()
}