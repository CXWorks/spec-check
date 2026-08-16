pub open spec fn psci_affinity_info_spec(result: PsciReturnCode, fid: UInt64, target_affinity: Bits64, lowest_affinity_level: UInt32, old_s: S, new_s: S) -> bool {
    (lowest_affinity_level != 0 ==> result == PSCI_INVALID_PARAMETERS)
    && (!MpidrIsUsed(old_s, target_affinity) ==> result == PSCI_INVALID_PARAMETERS)
    && (lowest_affinity_level == 0
        && MpidrIsUsed(old_s, target_affinity)
        && RecFromMpidr(old_s, target_affinity).flags.runnable == RUNNABLE
        ==> result == PSCI_SUCCESS)
    && (lowest_affinity_level == 0
        && MpidrIsUsed(old_s, target_affinity)
        && RecFromMpidr(old_s, target_affinity).flags.runnable == NOT_RUNNABLE
        ==> result == PSCI_OFF)
    && new_s == old_s
}