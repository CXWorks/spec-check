pub open spec fn psci_stat_count_spec(fid: UInt64, target_cpu: Mpidr, power_state: PowerState, result: UInt64, old_s: S, new_s: S) -> bool {
    (!StatFunctionsImplemented() ==> ResultEqual(result, NOT_SUPPORTED))
    && ((StatFunctionsImplemented() && !IsPresentNode(target_cpu)) ==> result == 0)
    && ((StatFunctionsImplemented() && !NodeSupportsState(target_cpu, power_state)) ==> result == 0)
    && ((StatFunctionsImplemented() && IsPresentNode(target_cpu) && NodeSupportsState(target_cpu, power_state)) ==> (
        ((fid == PSCI_STAT_COUNT) ==> result == StatCount(target_cpu, HighestLevelLocalState(power_state)))
        && ((fid == PSCI_STAT_RESIDENCY) ==> result == StatResidencyMicroseconds(target_cpu, HighestLevelLocalState(power_state)))
    ))
    && (new_s == old_s)
}
