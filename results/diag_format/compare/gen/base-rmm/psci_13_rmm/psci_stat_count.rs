pub open spec fn psci_stat_count_spec(result: UInt64, old_s: S, new_s: S) -> bool {
    (!StatFunctionsImplemented() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsPresentNode(target_cpu(old_s)) ==> result == 0)
    && (!NodeSupportsState(target_cpu(old_s), power_state(old_s)) ==> result == 0)
    && (IsOsInitiatedMode() ==> LastManLevelField(power_state(old_s)) is disregarded when the local state is selected)
    && (Only local low-power states are tracked)
    && (The statistics include low-power states entered by any PSCI call, including CPU_OFF, CPU_FREEZE and SYSTEM_SUSPEND)
    && (The result wraps when it overflows the resolution of the return register)
    && (old_s == new_s)
}