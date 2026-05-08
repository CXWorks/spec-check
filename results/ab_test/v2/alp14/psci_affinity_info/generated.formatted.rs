pub open spec fn psci_affinity_info_spec(
    result: PsciReturnCode,
    old_s: S,
    new_s: S,
    target_affinity: u64,
    lowest_affinity_level: u32,
) -> bool {
    // Failure condition: target_bound
    (lowest_affinity_level != 0 ==> result
        == PsciReturnCode::PSCI_INVALID_PARAMETERS)
    // Failure condition: target_match
     && (!MpidrIsUsed(old_s, target_affinity) ==> result
        == PsciReturnCode::PSCI_INVALID_PARAMETERS)
    // Success condition: runnable
     && (lowest_affinity_level == 0 && MpidrIsUsed(old_s, target_affinity) && RecFromMpidr(
        old_s,
        target_affinity,
    ).flags.runnable == RmmRecRunnableState::RUNNABLE ==> result
        == PsciReturnCode::PSCI_SUCCESS)
    // Success condition: not_runnable
     && (lowest_affinity_level == 0 && MpidrIsUsed(old_s, target_affinity) && RecFromMpidr(
        old_s,
        target_affinity,
    ).flags.runnable == RmmRecRunnableState::NOT_RUNNABLE ==> result
        == PsciReturnCode::PSCI_OFF)
    // No state changes
     && new_s == old_s
}