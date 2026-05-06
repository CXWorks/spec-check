pub open spec fn PSCI_AFFINITY_INFO_spec(
    old_s: S,
    new_s: S,
    target_affinity: u64,
    lowest_affinity_level: u32,
    result: PsciReturnCode,
) -> bool {
    let target_rec = RecFromMpidr(old_s, target_affinity);

    (lowest_affinity_level != 0 ==> result == PSCI_INVALID_PARAMETERS) && (!MpidrIsUsed(
        old_s,
        target_affinity,
    ) ==> result == PSCI_INVALID_PARAMETERS) && (lowest_affinity_level == 0 && MpidrIsUsed(
        old_s,
        target_affinity,
    ) ==> ((target_rec.flags.runnable == RUNNABLE() && result == PSCI_SUCCESS()) || (
    target_rec.flags.runnable == NOT_RUNNABLE() && result == PSCI_OFF()))) && (new_s == old_s)
}