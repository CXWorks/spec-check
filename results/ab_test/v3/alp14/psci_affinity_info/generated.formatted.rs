pub open spec fn psci_affinity_info_spec(
    result: PsciReturnCode,
    target_affinity: u64,
    lowest_affinity_level: u32,
    old_s: S,
    new_s: S,
) -> bool {
    (lowest_affinity_level != 0 ==> result == PSCI_INVALID_PARAMETERS) && (!MpidrIsUsed(
        target_affinity,
    ) ==> result == PSCI_INVALID_PARAMETERS) && (MpidrIsUsed(target_affinity)
        && lowest_affinity_level == 0 && RecFromMpidr(target_affinity).flags.runnable == RUNNABLE
        ==> result == PSCI_SUCCESS) && (MpidrIsUsed(target_affinity) && lowest_affinity_level == 0
        && RecFromMpidr(target_affinity).flags.runnable == NOT_RUNNABLE ==> result == PSCI_OFF)
}